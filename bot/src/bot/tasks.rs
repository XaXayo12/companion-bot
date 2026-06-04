use std::sync::Arc;
use std::time::Duration;

use azalea::block::BlockTrait;
use azalea::container::ContainerClientExt;
use azalea::inventory::ItemStack;
use azalea::inventory::operations::ThrowClick;
use azalea::pathfinder::PathfinderClientExt;
use azalea::pathfinder::goals::RadiusGoal;
use azalea::prelude::*;
use azalea::registry::builtin::BlockKind;
use azalea::{BlockPos, Vec3};

use crate::bot::handler::whisper;
use crate::bot::{dist, human, identity, world_scan};
use crate::shared::{BotCtx, Mode};

const REACH: f64 = 4.0;
/// Vanilla survival block-interaction reach, in blocks. The bot never mines a
/// block farther than this from its eyes, so its reach stays legit.
const VANILLA_BLOCK_REACH: f64 = 4.5;
const CRITICAL_HEALTH: f32 = 6.0;

/// Log to the mod console and whisper the same line back to the player who
/// started this job, so they see progress and the reason on failure in game.
fn report(bot: &Client, ctx: &Arc<BotCtx>, text: &str) {
    let who = { ctx.rt.lock().command_sender.clone() };
    ctx.log(text);
    whisper(bot, who.as_deref(), text);
}

// True when the bot is in danger and long jobs should pause.
fn unsafe_now(bot: &Client) -> bool {
    bot.health() <= CRITICAL_HEALTH
}

async fn wait_until_safe(bot: &Client) {
    while unsafe_now(bot) {
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

// Mine every block in the box between the two selected points, top layer first.
pub async fn mine_area(bot: Client, ctx: Arc<BotCtx>) {
    let (Some(a), Some(b)) = ({ ctx.rt.lock().sel1 }, { ctx.rt.lock().sel2 }) else {
        ctx.rt.lock().mining_active = false;
        return;
    };

    report(&bot, &ctx, "Mining started.");

    let (min_x, max_x) = (a.x.min(b.x), a.x.max(b.x));
    let (min_y, max_y) = (a.y.min(b.y), a.y.max(b.y));
    let (min_z, max_z) = (a.z.min(b.z), a.z.max(b.z));

    'outer: for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            for z in min_z..=max_z {
                if ctx.rt.lock().mode != Mode::Mine {
                    break 'outer;
                }
                wait_until_safe(&bot).await;

                let block = BlockPos::new(x, y, z);
                let Some(block_id) = block_id_at(&bot, block) else {
                    continue;
                };
                if block_id.ends_with("air") {
                    continue;
                }

                let center = Vec3 {
                    x: x as f64 + 0.5,
                    y: y as f64 + 0.5,
                    z: z as f64 + 0.5,
                };
                // Walk right up to the block first, then only mine it if it is
                // within vanilla survival reach. If the bot cannot get close
                // enough (blocked or unreachable), skip it instead of breaking a
                // block from across the room.
                bot.goto(RadiusGoal::new(center, 2.5)).await;
                if dist(bot.eye_position(), center) > VANILLA_BLOCK_REACH {
                    continue;
                }

                // Hold the right tool for this block (pickaxe / axe / shovel).
                if let Some(slot) = identity::best_tool_slot_for(&bot, &block_id) {
                    bot.set_selected_hotbar_slot(slot);
                }
                bot.look_at(center);
                human::pause(60, 160).await;
                bot.mine(block).await;
                human::pause(40, 120).await;
            }
        }
    }

    report(&bot, &ctx, "Mining finished.");
    let mut rt = ctx.rt.lock();
    rt.mining_active = false;
    if rt.mode == Mode::Mine {
        rt.mode = Mode::Idle;
    }
}

/// The registry id of the block at `pos` (e.g. "stone", "oak_log"), or `None`
/// when the chunk is not loaded.
fn block_id_at(bot: &Client, pos: BlockPos) -> Option<String> {
    let world = bot.world();
    let instance = world.read();
    instance
        .get_block_state(pos)
        .map(|state| Box::<dyn BlockTrait>::from(state).id().to_string())
}

// Detect stasis chambers, then activate one (no arg = just list them).
pub async fn pull_stasis(bot: Client, ctx: Arc<BotCtx>, arg: String) {
    refresh_stasis(&bot, &ctx);

    let list = ctx.rt.lock().stasis.clone();
    if list.is_empty() {
        report(&bot, &ctx, "No stasis chamber found nearby.");
        ctx.rt.lock().pull_active = false;
        return;
    }

    let chosen = match arg.as_str() {
        "manual" => {
            for s in &list {
                let owner = s.owner.clone().unwrap_or_else(|| "unknown".into());
                ctx.log(&format!(
                    "Stasis {} at {} {} {} (owner: {})",
                    s.id, s.pos.x, s.pos.y, s.pos.z, owner
                ));
            }
            report(
                &bot,
                &ctx,
                &format!(
                    "Found {} stasis chamber(s). Use !pull <id> or !pull <owner>.",
                    list.len()
                ),
            );
            ctx.rt.lock().pull_active = false;
            return;
        }
        "health_low" => list.first().cloned(),
        other => {
            if let Ok(id) = other.parse::<u32>() {
                list.iter().find(|s| s.id == id).cloned()
            } else {
                list.iter()
                    .find(|s| {
                        s.owner.as_deref().map(|o| o.eq_ignore_ascii_case(other)) == Some(true)
                    })
                    .cloned()
            }
        }
    };

    let Some(stasis) = chosen else {
        report(&bot, &ctx, "That stasis was not found.");
        ctx.rt.lock().pull_active = false;
        return;
    };

    let Some(trapdoor) = find_trapdoor_near(&bot, stasis.pos) else {
        report(&bot, &ctx, "No trapdoor or lever found next to that pearl.");
        ctx.rt.lock().pull_active = false;
        return;
    };

    for attempt in 0..5 {
        wait_until_safe(&bot).await;

        let center = Vec3 {
            x: trapdoor.x as f64 + 0.5,
            y: trapdoor.y as f64 + 0.5,
            z: trapdoor.z as f64 + 0.5,
        };
        bot.goto(RadiusGoal::new(center, REACH as f32)).await;
        bot.look_at(center);

        // Right-click the trapdoor to activate the stasis mechanism.
        bot.block_interact(trapdoor);

        tokio::time::sleep(Duration::from_secs(5)).await;

        refresh_stasis(&bot, &ctx);
        let still_there = ctx.rt.lock().stasis.iter().any(|s| s.pos == stasis.pos);
        if !still_there {
            report(&bot, &ctx, "Pulled successfully.");
            ctx.rt.lock().pull_active = false;
            return;
        }
        ctx.log(&format!(
            "Pull attempt {} did not work, retrying...",
            attempt + 1
        ));
    }

    report(&bot, &ctx, "Could not pull after several attempts.");
    ctx.rt.lock().pull_active = false;
}

fn refresh_stasis(bot: &Client, ctx: &Arc<BotCtx>) {
    let found = world_scan::find_stasis(bot);
    let mut rt = ctx.rt.lock();
    let mut list = Vec::new();
    for (pos, owner) in found {
        let existing = rt.stasis.iter().find(|s| s.pos == pos);
        let id = match existing {
            Some(s) => s.id,
            None => {
                let id = rt.next_stasis_id;
                rt.next_stasis_id += 1;
                id
            }
        };
        list.push(crate::shared::Stasis { id, pos, owner });
    }
    rt.stasis = list;
}

// Find the nearest chest, path to it, open it, and shift-click non-essential
// items into the container. Uses the verified azalea container API.
pub async fn deposit_items(bot: Client, ctx: Arc<BotCtx>) {
    ctx.log("Looking for a chest to deposit into...");

    let chest = {
        let world = bot.world();
        let instance = world.read();
        let states: azalea::block::BlockStates =
            (&[BlockKind::Chest, BlockKind::TrappedChest][..]).into();
        instance.find_block(bot.position(), &states)
    };

    let Some(chest) = chest else {
        report(&bot, &ctx, "No chest found nearby.");
        ctx.rt.lock().deposit_active = false;
        return;
    };

    wait_until_safe(&bot).await;

    let center = Vec3 {
        x: chest.x as f64 + 0.5,
        y: chest.y as f64 + 0.5,
        z: chest.z as f64 + 0.5,
    };
    bot.goto(RadiusGoal::new(center, REACH as f32)).await;
    bot.look_at(center);

    let Some(container) = bot.open_container_at(chest).await else {
        report(&bot, &ctx, "Could not open the chest.");
        ctx.rt.lock().deposit_active = false;
        return;
    };

    // The open container menu places the chest's own slots first, then the
    // player's inventory. Collect the trash slots first (so we don't hold the
    // menu lock across the awaits below), then move them one at a time with
    // human-like pauses instead of instantly.
    let mut targets = Vec::new();
    if let Some(menu) = container.menu() {
        let player_range = menu.player_slots_range();
        let slots = menu.slots();
        for index in player_range {
            if let Some(ItemStack::Present(item)) = slots.get(index)
                && identity::is_trash(item.kind)
            {
                targets.push(index);
            }
        }
    }

    // A short beat after opening, as if reading the chest.
    human::pause(280, 620).await;
    let mut moved = 0u32;
    for index in targets {
        container.shift_click(index);
        moved += 1;
        human::pause(110, 260).await;
    }

    human::pause(150, 340).await;
    container.close();
    ctx.log(&format!("Deposited {moved} item stack(s)."));
    ctx.rt.lock().deposit_active = false;
}

// Drop every "trash" item stack from the player's inventory using the verified
// ThrowClick::All drop operation.
pub async fn drop_trash(bot: Client, ctx: Arc<BotCtx>) {
    ctx.log("Dropping trash items...");

    let inv = bot.get_inventory();
    // Collect trash slots first so we don't hold the menu lock across awaits.
    let mut targets = Vec::new();
    if let Some(menu) = inv.menu() {
        let slots = menu.slots();
        for index in identity::PLAYER_STORAGE_START..=identity::PLAYER_STORAGE_END {
            if let Some(ItemStack::Present(item)) = slots.get(index)
                && identity::is_trash(item.kind)
            {
                targets.push(index);
            }
        }
    }

    let mut dropped = 0u32;
    for index in targets {
        inv.click(ThrowClick::All { slot: index as u16 });
        dropped += 1;
        human::pause(90, 220).await;
    }
    inv.close();

    ctx.log(&format!("Dropped {dropped} trash stack(s)."));
    ctx.rt.lock().drop_trash_active = false;
}

fn find_trapdoor_near(bot: &Client, pearl: BlockPos) -> Option<BlockPos> {
    let world = bot.world();
    let instance = world.read();
    for dx in -2..=2 {
        for dy in -3..=2 {
            for dz in -2..=2 {
                let pos = BlockPos::new(pearl.x + dx, pearl.y + dy, pearl.z + dz);
                if let Some(state) = instance.get_block_state(pos) {
                    // Match any trapdoor by its registry id ("oak_trapdoor",
                    // "iron_trapdoor", ...) instead of a Debug string.
                    let id = Box::<dyn BlockTrait>::from(state).id();
                    if id.ends_with("trapdoor") {
                        return Some(pos);
                    }
                }
            }
        }
    }
    None
}
