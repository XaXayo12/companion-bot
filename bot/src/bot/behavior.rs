use std::sync::Arc;

use azalea::entity::Physics;
use azalea::local_player::Hunger;
use azalea::pathfinder::PathfinderClientExt;
use azalea::pathfinder::goals::{BlockPosGoal, RadiusGoal};
use azalea::prelude::*;
use azalea::registry::builtin::ItemKind as Item;
use azalea::{SprintDirection, Vec3, WalkDirection};

use crate::bot::handler::SwarmState;
use crate::bot::tasks;
use crate::bot::world_scan;
use crate::bot::{dist, human, identity};
use crate::shared::{BotCtx, Mode};

/// Distance the protect owner is followed at when no enemy is in range.
const PROTECT_DISTANCE: f32 = 3.0;

/// Send the bot toward `target` without flooding the pathfinder.
///
/// Azalea's pathfinder recomputes a whole path on every `start_goto`. Calling it
/// each tick (as the old follow/combat code did) made the bot stutter, freeze,
/// and spam "pathfinder timeout"/"path obstructed" forever. Here we only issue a
/// new path when we are not already walking, or when the target has actually
/// moved away from the goal we last sent. Otherwise we let the existing path run.
fn path_towards(bot: &Client, ctx: &Arc<BotCtx>, target: Vec3, radius: f32) {
    if !bot.is_goto_target_reached() {
        // Already walking a path: only redirect if the target shifted notably.
        let moved = {
            let rt = ctx.rt.lock();
            rt.last_goal.is_none_or(|g| dist(g, target) > 1.5)
        };
        if moved {
            bot.start_goto(RadiusGoal::new(target, radius));
            ctx.rt.lock().last_goal = Some(target);
        }
        return;
    }
    // Idle: only start a path if we are genuinely too far away. This keeps the
    // bot standing calmly next to the player instead of re-pathing every tick.
    if dist(bot.position(), target) > radius as f64 + 0.5 {
        bot.start_goto(RadiusGoal::new(target, radius));
        ctx.rt.lock().last_goal = Some(target);
    }
}

/// Stop any active path and forget the last goal.
fn halt(bot: &Client, ctx: &Arc<BotCtx>) {
    if !bot.is_goto_target_reached() {
        bot.stop_pathfinding();
    }
    ctx.rt.lock().last_goal = None;
}

/// True when the bot's body is in water.
fn in_water(bot: &Client) -> bool {
    bot.get_component::<Physics>().is_some_and(|p| p.is_in_water())
}

/// Swim toward a target. The pathfinder deliberately avoids liquids, so when the
/// bot is actually in water we take over: face the target, swim forward, and
/// hold jump so the bot rises to the surface instead of sinking. This is what
/// lets the bot follow a player across rivers, lakes, and oceans.
fn swim_towards(bot: &Client, ctx: &Arc<BotCtx>, target: Vec3) {
    // Drop any path the pathfinder left stranded at the water's edge so that,
    // once we climb out, `path_towards` starts a fresh land route.
    ctx.rt.lock().last_goal = None;
    bot.look_at(Vec3 {
        x: target.x,
        y: target.y + 0.4,
        z: target.z,
    });
    bot.walk(WalkDirection::Forward);
    bot.jump();
}

/// Go to a target the smart way: swim if we're in water, otherwise pathfind.
fn move_towards(bot: &Client, ctx: &Arc<BotCtx>, target: Vec3, radius: f32) {
    if in_water(bot) {
        swim_towards(bot, ctx, target);
    } else {
        path_towards(bot, ctx, target, radius);
    }
}

/// Turn the head toward `target` smoothly, by at most a bounded number of
/// degrees per tick, instead of snapping instantly. This is what stops the
/// jarring 360-degree flicks and abrupt head turns.
fn smooth_look_at(bot: &Client, target: Vec3) {
    let eye = bot.eye_position();
    let dx = target.x - eye.x;
    let dy = target.y - eye.y;
    let dz = target.z - eye.z;
    let horizontal = (dx * dx + dz * dz).sqrt();
    if horizontal < 1e-6 {
        return;
    }
    let target_yaw = (-dx).atan2(dz).to_degrees() as f32;
    let target_pitch = (-dy).atan2(horizontal).to_degrees() as f32;

    let (current_yaw, current_pitch) = bot.direction();
    const MAX_STEP_DEG: f32 = 32.0;

    // Shortest signed yaw difference, wrapped into [-180, 180].
    let mut yaw_delta = (target_yaw - current_yaw).rem_euclid(360.0);
    if yaw_delta > 180.0 {
        yaw_delta -= 360.0;
    }
    let new_yaw = current_yaw + yaw_delta.clamp(-MAX_STEP_DEG, MAX_STEP_DEG);
    let pitch_delta = (target_pitch - current_pitch).clamp(-MAX_STEP_DEG, MAX_STEP_DEG);
    let new_pitch = (current_pitch + pitch_delta).clamp(-90.0, 90.0);

    bot.set_direction(new_yaw, new_pitch);
}

const EAT_DURATION_TICKS: u32 = 36;
const TELEMETRY_EVERY_TICKS: u32 = 10;
const ANTI_AFK_EVERY_TICKS: u32 = 200;
const EQUIP_EVERY_TICKS: u32 = 20;
/// How often (in ticks, 20/s) the "I can't see you" warning may repeat.
const CANT_SEE_EVERY_TICKS: u32 = 100;
/// Vanilla survival attack reach (entity interaction range), in blocks. The bot
/// never swings at anything farther than this, so its reach stays legit.
const VANILLA_ATTACK_REACH: f64 = 3.0;

pub fn tick(bot: &Client, ctx: &Arc<BotCtx>) {
    send_telemetry(bot, ctx);

    // Send any chat line the GUI say bar queued.
    if let Some(line) = ctx.rt.lock().pending_say.take() {
        bot.chat(line);
    }

    // 0a. Emergency: clutch a dangerous fall with a water bucket (MLG).
    if auto_mlg(bot, ctx) {
        return;
    }

    // 0b. Emergency: disconnect if health is critically low (and feature is on).
    if auto_disconnect(bot, ctx) {
        return;
    }

    // 1. Survival comes first.
    if auto_eat(bot, ctx) {
        return;
    }

    // Keep gear up to date (offhand totem + best armor).
    maintain_equipment(bot, ctx);

    // Apply movement modifiers (sprint / sneak).
    apply_movement_flags(bot, ctx);

    // 2. Start long async jobs if requested (mining / stasis pull / deposit / drop).
    maybe_start_tasks(bot, ctx);

    // 3. Combat: defend or farm.
    if combat(bot, ctx) {
        return;
    }

    // 4. The active movement task.
    let mode = ctx.rt.lock().mode;
    match mode {
        Mode::Follow => follow(bot, ctx),
        Mode::Protect => protect_follow(bot, ctx),
        Mode::Goto => goto(bot, ctx),
        Mode::Hold => {
            halt(bot, ctx);
        }
        _ => {}
    }

    anti_afk(bot, ctx);
}

// Disconnect cleanly when health drops to/below the configured threshold.
fn auto_disconnect(bot: &Client, ctx: &Arc<BotCtx>) -> bool {
    let (on, threshold) = {
        let rt = ctx.rt.lock();
        (rt.auto_disconnect, rt.auto_disconnect_health)
    };
    if !on {
        return false;
    }
    if (bot.health() as f64) <= threshold {
        ctx.log(&format!(
            "Health {:.1} <= {:.1}, disconnecting.",
            bot.health(),
            threshold
        ));
        ctx.status("disconnected");
        bot.disconnect();
        return true;
    }
    false
}

// Emergency water-bucket clutch (MLG): when the bot is falling far enough to get
// hurt and is holding a water bucket, it aims the bucket straight down so the
// water lands under it, then picks the water back up once it is safely down.
fn auto_mlg(bot: &Client, ctx: &Arc<BotCtx>) -> bool {
    let Some(physics) = bot.get_component::<Physics>() else {
        return false;
    };
    let was_clutching = ctx.rt.lock().mlg_placed;

    let falling_danger = !physics.on_ground()
        && physics.velocity.y < -0.4
        && physics.fall_distance >= 3.0
        && !in_water(bot);

    if falling_danger
        && let Some(slot) = identity::hotbar_slot(bot, identity::is_water_bucket)
    {
        if !was_clutching {
            ctx.log("Clutching with water (MLG).");
        }
        // Keep aiming the bucket down; the server places the water source the
        // moment the ground comes within reach.
        bot.set_selected_hotbar_slot(slot);
        let (yaw, _) = bot.direction();
        bot.set_direction(yaw, 90.0);
        bot.start_use_item();
        ctx.rt.lock().mlg_placed = true;
        return true;
    }

    if was_clutching && (physics.on_ground() || in_water(bot)) {
        // Landed safely: scoop the water back up so we keep the bucket.
        if let Some(slot) = identity::hotbar_slot(bot, identity::is_empty_bucket) {
            bot.set_selected_hotbar_slot(slot);
            let (yaw, _) = bot.direction();
            bot.set_direction(yaw, 90.0);
            bot.start_use_item();
        }
        ctx.rt.lock().mlg_placed = false;
    }
    false
}

// Keep a totem in the offhand and equip the best armor, throttled.
fn maintain_equipment(bot: &Client, ctx: &Arc<BotCtx>) {
    let (auto_totem, auto_armor) = {
        let mut rt = ctx.rt.lock();
        if rt.equip_cooldown > 0 {
            rt.equip_cooldown -= 1;
            return;
        }
        rt.equip_cooldown = EQUIP_EVERY_TICKS;
        (rt.auto_totem, rt.auto_armor)
    };

    // Offhand priority: a Totem of Undying if auto-totem wants one, otherwise a
    // shield so the bot is never empty-handed in its offhand.
    let offhand = identity::offhand_item(bot);
    let totem_slot = if auto_totem && offhand != Some(Item::TotemOfUndying) {
        identity::find_storage_slot(bot, identity::is_totem)
    } else {
        None
    };
    if let Some(slot) = totem_slot {
        // Pick up the totem, place it in the offhand, via the player inventory.
        let inv = bot.get_inventory();
        inv.left_click(slot);
        inv.left_click(identity::PLAYER_OFFHAND);
        inv.left_click(slot);
    } else if !(auto_totem && offhand == Some(Item::TotemOfUndying))
        && !offhand.is_some_and(identity::is_shield)
        && let Some(slot) = identity::find_storage_slot(bot, identity::is_shield)
    {
        let inv = bot.get_inventory();
        inv.left_click(slot);
        inv.left_click(identity::PLAYER_OFFHAND);
        inv.left_click(slot);
    }

    if auto_armor {
        for want in [
            identity::ArmorSlot::Helmet,
            identity::ArmorSlot::Chestplate,
            identity::ArmorSlot::Leggings,
            identity::ArmorSlot::Boots,
        ] {
            let equipped_rank = identity::equipped_armor(bot, want)
                .and_then(identity::armor_info)
                .map(|(_, r)| r)
                .unwrap_or(0);
            if let Some((slot, rank)) = identity::best_armor_in_storage(bot, want)
                && rank > equipped_rank
            {
                let inv = bot.get_inventory();
                let target = identity::armor_target_slot(want);
                inv.left_click(slot);
                inv.left_click(target);
                inv.left_click(slot);
            }
        }
    }
}

// Apply sprint/sneak flags.
//
// The pathfinder moves the bot by writing its own walk/sprint/jump events every
// tick. If we also write walk events here we fight it: the bot freezes mid-path
// and the pathfinder logs "obstructed" forever. So while a path is active we do
// not touch movement at all and let the pathfinder drive. Manual sprint/walk is
// only meaningful when the bot is idle with no path.
//
// `set_crouching` is a verified persistent toggle and is safe to mirror anytime.
fn apply_movement_flags(bot: &Client, ctx: &Arc<BotCtx>) {
    let (sprinting, sneaking) = {
        let rt = ctx.rt.lock();
        (rt.sprinting, rt.sneaking)
    };

    bot.set_crouching(sneaking);

    // Never override the pathfinder while it is walking the bot somewhere, and
    // never fight the manual swim handler while we're in water.
    if !bot.is_goto_target_reached() || in_water(bot) {
        return;
    }

    if sprinting {
        bot.sprint(SprintDirection::Forward);
    } else {
        bot.walk(WalkDirection::None);
    }
}

fn send_telemetry(bot: &Client, ctx: &Arc<BotCtx>) {
    let due = {
        let mut rt = ctx.rt.lock();
        if rt.telemetry_cooldown > 0 {
            rt.telemetry_cooldown -= 1;
            false
        } else {
            rt.telemetry_cooldown = TELEMETRY_EVERY_TICKS;
            true
        }
    };
    if !due {
        return;
    }
    let pos = bot.position();
    let health = bot.health();
    // Real food level from the game (falls back to full if not loaded yet).
    let food = bot.get_component::<Hunger>().map_or(20, |h| h.food);
    ctx.telemetry(health, food, pos.x, pos.y, pos.z);
}

// Always on: the bot eats when its real food level drops to/below the
// configured threshold (read live from the game's Hunger component), the same
// way a player eats when the hunger bar gets low. Returns true while the bot is
// mid-bite so the tick loop holds it still until the animation finishes.
fn auto_eat(bot: &Client, ctx: &Arc<BotCtx>) -> bool {
    {
        let mut rt = ctx.rt.lock();
        if rt.eat_cooldown > 0 {
            rt.eat_cooldown -= 1;
            // Still chewing: keep holding still until the cooldown elapses.
            return rt.eat_cooldown > 0;
        }
    }

    let threshold = ctx.rt.lock().eat_threshold;
    // Only eat when actually hungry. If hunger isn't readable yet, assume full.
    let food = bot.get_component::<Hunger>().map_or(20, |h| h.food);
    if food > threshold {
        return false;
    }

    let Some(slot) = identity::food_slot(bot) else {
        return false;
    };
    bot.set_selected_hotbar_slot(slot);
    bot.start_use_item();
    // Hold still for the eating animation, then re-check hunger next time.
    ctx.rt.lock().eat_cooldown = EAT_DURATION_TICKS;
    true
}

fn combat(bot: &Client, ctx: &Arc<BotCtx>) -> bool {
    let (active, range, stay_still) = {
        let rt = ctx.rt.lock();
        let active = rt.killaura_on || rt.mode == Mode::Protect;
        (active, rt.killaura_range, rt.killaura_stay_still)
    };
    if !active {
        return false;
    }

    let Some((entity, target_pos)) = world_scan::nearest_hostile(bot, range + 0.5) else {
        return false;
    };

    // Hold the best melee weapon: a sword, or an axe if it hits harder.
    if let Some(slot) = identity::best_weapon_slot(bot) {
        bot.set_selected_hotbar_slot(slot);
    }

    // Aim a touch off the exact centre, and turn the head smoothly rather than
    // snapping, so the bot does not flick around like an aimbot.
    let aim = azalea::Vec3 {
        x: target_pos.x + human::aim_jitter(),
        y: target_pos.y + 1.3,
        z: target_pos.z + human::aim_jitter(),
    };
    smooth_look_at(bot, aim);

    // Only ever swing within vanilla survival reach, never farther, even if the
    // configured killaura range is larger. This keeps the bot's reach legit.
    let attack_reach = range.min(VANILLA_ATTACK_REACH);
    let in_reach = dist(bot.eye_position(), aim) <= attack_reach;
    if in_reach {
        // Stand still and trade hits like a player: stop any walk-up path so we
        // don't slide past the target, and only swing on a full cooldown (never a
        // useless spam-click). `human::chance` adds a tick or two of reaction
        // jitter so swings are not frame-perfect.
        halt(bot, ctx);
        if !bot.has_attack_cooldown() && human::chance(0.55) {
            bot.attack(entity);
        }
        return true;
    }

    // Out of reach: close the distance (this is the "come to me and defend"
    // behaviour), throttled so we don't restart the path every tick, and able to
    // swim if the fight crosses water.
    if !stay_still {
        move_towards(bot, ctx, target_pos, (attack_reach - 0.5).max(1.0) as f32);
        return true;
    }
    false
}

/// The player a follow/protect command should track: the explicit target if one
/// was given, otherwise the owner the mod reported (your own Minecraft name).
/// Returns `None` only when no target was given and no owner is known yet, so a
/// GUI "Follow" with no name still works the moment the mod has linked.
fn tracked_name(bot: &Client, explicit: Option<String>) -> Option<String> {
    if explicit.is_some() {
        return explicit;
    }
    let owner = bot.resource::<SwarmState>().shared.owner();
    if owner.is_empty() { None } else { Some(owner) }
}

/// Warn — at most once every few seconds — that the bot cannot see `name`. This
/// is the honest, coherent answer when you press Follow/Protect but you are on a
/// different server than the bot, or simply out of its render distance: instead
/// of standing there silently, the bot tells you why nothing is happening.
fn warn_cant_see(ctx: &Arc<BotCtx>, name: &str) {
    {
        let mut rt = ctx.rt.lock();
        if rt.cant_see_cooldown > 0 {
            rt.cant_see_cooldown -= 1;
            return;
        }
        rt.cant_see_cooldown = CANT_SEE_EVERY_TICKS;
    }
    ctx.error(&format!(
        "I can't see {name}. Make sure the bot is on the same server as you and you're within render distance."
    ));
}

fn follow(bot: &Client, ctx: &Arc<BotCtx>) {
    // Runs every tick (like Azalea's own example bot). `move_towards` already
    // throttles how often a new path is computed, and running every tick is what
    // lets swimming work: `swim_towards` must press jump each tick to stay afloat
    // instead of sinking between scans.
    let (explicit, distance) = {
        let rt = ctx.rt.lock();
        (rt.follow_target.clone(), rt.follow_distance)
    };
    let Some(name) = tracked_name(bot, explicit) else {
        return;
    };
    match world_scan::find_player(bot, &name) {
        // On land the pathfinder routes around lava and drops; in water we swim.
        Some((_, pos)) => {
            ctx.rt.lock().cant_see_cooldown = 0;
            move_towards(bot, ctx, pos, distance.max(1.0) as f32);
        }
        None => warn_cant_see(ctx, &name),
    }
}

// Protect: when no enemy is in reach (combat already handled that), stay close
// to the protected player so the bot defends them wherever they go.
fn protect_follow(bot: &Client, ctx: &Arc<BotCtx>) {
    let explicit = { ctx.rt.lock().protect_target.clone() };
    let Some(name) = tracked_name(bot, explicit) else {
        return;
    };
    match world_scan::find_player(bot, &name) {
        Some((_, pos)) => {
            ctx.rt.lock().cant_see_cooldown = 0;
            move_towards(bot, ctx, pos, PROTECT_DISTANCE);
        }
        None => warn_cant_see(ctx, &name),
    }
}

// Path to the stored goto target, then drop back to Idle once close enough.
fn goto(bot: &Client, ctx: &Arc<BotCtx>) {
    let target = {
        let rt = ctx.rt.lock();
        rt.goto_target
    };
    let Some(target) = target else {
        ctx.rt.lock().mode = Mode::Idle;
        return;
    };
    let here = bot.position();
    let center = azalea::Vec3 {
        x: target.x as f64 + 0.5,
        y: target.y as f64,
        z: target.z as f64 + 0.5,
    };
    if dist(here, center) <= 1.5 {
        halt(bot, ctx);
        let mut rt = ctx.rt.lock();
        rt.goto_target = None;
        if rt.mode == Mode::Goto {
            rt.mode = Mode::Idle;
        }
        drop(rt);
        ctx.log("Arrived at destination.");
        return;
    }
    // Only (re)issue the path when we are not already walking one, so we don't
    // recompute it every tick.
    if bot.is_goto_target_reached() {
        bot.start_goto(BlockPosGoal(target));
    }
}

fn maybe_start_tasks(bot: &Client, ctx: &Arc<BotCtx>) {
    let start_mine;
    let pull_arg;
    let start_deposit;
    let start_drop;
    {
        let mut rt = ctx.rt.lock();
        start_mine = rt.mode == Mode::Mine && !rt.mining_active;
        if start_mine {
            rt.mining_active = true;
        }
        pull_arg = if !rt.pull_active {
            rt.pull_request.take()
        } else {
            rt.pull_request = None;
            None
        };
        if pull_arg.is_some() {
            rt.pull_active = true;
        }

        start_deposit = rt.deposit_request && !rt.deposit_active;
        rt.deposit_request = false;
        if start_deposit {
            rt.deposit_active = true;
        }

        start_drop = rt.drop_trash_request && !rt.drop_trash_active;
        rt.drop_trash_request = false;
        if start_drop {
            rt.drop_trash_active = true;
        }
    }

    if start_mine {
        let bot = bot.clone();
        let ctx = ctx.clone();
        tokio::task::spawn_local(async move {
            tasks::mine_area(bot, ctx).await;
        });
    }
    if let Some(arg) = pull_arg {
        let bot = bot.clone();
        let ctx = ctx.clone();
        tokio::task::spawn_local(async move {
            tasks::pull_stasis(bot, ctx, arg).await;
        });
    }
    if start_deposit {
        let bot = bot.clone();
        let ctx = ctx.clone();
        tokio::task::spawn_local(async move {
            tasks::deposit_items(bot, ctx).await;
        });
    }
    if start_drop {
        let bot = bot.clone();
        let ctx = ctx.clone();
        tokio::task::spawn_local(async move {
            tasks::drop_trash(bot, ctx).await;
        });
    }
}

fn anti_afk(bot: &Client, ctx: &Arc<BotCtx>) {
    let mut rt = ctx.rt.lock();
    rt.anti_afk += 1;
    // Glance after a slightly random gap, not on an exact clock.
    if rt.anti_afk >= ANTI_AFK_EVERY_TICKS + human::ticks(0, 40) {
        rt.anti_afk = 0;
        drop(rt);
        // A small, subtle head turn in a random direction. Never a full spin.
        let (yaw, pitch) = bot.direction();
        bot.set_direction(yaw + human::turn_degrees(), pitch);
    }
}
