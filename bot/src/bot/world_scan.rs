// This module is the only place that queries Azalea's ECS directly.

use azalea::block::BlockTrait;
use azalea::ecs::prelude::*;
use azalea::entity::{EntityKindComponent, Position};
use azalea::player::GameProfileComponent;
use azalea::prelude::*;
use azalea::registry::builtin::EntityKind as RegistryEntityKind;
use azalea::world::{InstanceName, MinecraftEntityId};
use azalea::{BlockPos, Vec3};

use crate::bot::dist;
use crate::bot::identity::is_hostile;

pub struct Scanned {
    pub entity: Entity,
    pub pos: Vec3,
    pub kind: RegistryEntityKind,
    pub name: Option<String>,
}

pub fn scan_world(bot: &Client) -> Vec<Scanned> {
    let me = bot.entity;
    let my_instance = bot.get_component::<InstanceName>();

    let mut ecs = bot.ecs.lock();
    let mut query = ecs.query_filtered::<(
        Entity,
        &Position,
        &EntityKindComponent,
        &InstanceName,
        Option<&GameProfileComponent>,
    ), With<MinecraftEntityId>>();

    let mut out = Vec::new();
    for (entity, pos, kind, instance, profile) in query.iter(&ecs) {
        if entity == me {
            continue;
        }
        if let Some(mine) = &my_instance
            && instance != mine {
                continue;
            }
        out.push(Scanned {
            entity,
            pos: Vec3 {
                x: pos.x,
                y: pos.y,
                z: pos.z,
            },
            kind: kind.0,
            name: profile.map(|p| p.name.clone()),
        });
    }
    out
}

pub fn find_player(bot: &Client, name: &str) -> Option<(Entity, Vec3)> {
    scan_world(bot).into_iter().find_map(|e| {
        if matches!(e.kind, RegistryEntityKind::Player)
            && e.name.as_deref().map(|n| n.eq_ignore_ascii_case(name)) == Some(true)
        {
            Some((e.entity, e.pos))
        } else {
            None
        }
    })
}

pub fn nearest_hostile(bot: &Client, max_range: f64) -> Option<(Entity, Vec3)> {
    let eye = bot.eye_position();
    let mut best: Option<(Entity, Vec3, f64)> = None;
    for e in scan_world(bot) {
        if !is_hostile(e.kind) {
            continue;
        }
        let target = Vec3 {
            x: e.pos.x,
            y: e.pos.y + 1.3,
            z: e.pos.z,
        };
        let d = dist(eye, target);
        if d > max_range {
            continue;
        }
        if !has_line_of_sight(bot, eye, target) {
            continue;
        }
        if best.as_ref().is_none_or(|(_, _, bd)| d < *bd) {
            best = Some((e.entity, e.pos, d));
        }
    }
    best.map(|(entity, pos, _)| (entity, pos))
}

pub fn has_line_of_sight(bot: &Client, from: Vec3, to: Vec3) -> bool {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let dz = to.z - from.z;
    let len = (dx * dx + dy * dy + dz * dz).sqrt();
    if len < 0.001 {
        return true;
    }
    let steps = (len / 0.25).ceil() as i32;

    let world = bot.world();
    let instance = world.read();

    for i in 1..steps {
        let t = i as f64 / steps as f64;
        let px = (from.x + dx * t).floor() as i32;
        let py = (from.y + dy * t).floor() as i32;
        let pz = (from.z + dz * t).floor() as i32;
        let block = BlockPos::new(px, py, pz);
        if let Some(state) = instance.get_block_state(block)
            && !state.is_air() {
                return false;
            }
    }
    true
}

// A stasis chamber = a thrown ender pearl held in place with soul sand below it.
// We detect ender-pearl entities that have soul sand within a few blocks under
// them. Returns (pearl position, optional thrower name).
pub fn find_stasis(bot: &Client) -> Vec<(BlockPos, Option<String>)> {
    let world = bot.world();
    let instance = world.read();

    let mut out = Vec::new();
    for e in scan_world(bot) {
        if !matches!(e.kind, RegistryEntityKind::EnderPearl) {
            continue;
        }
        let base = BlockPos::new(
            e.pos.x.floor() as i32,
            e.pos.y.floor() as i32,
            e.pos.z.floor() as i32,
        );
        let mut has_soul_sand = false;
        for dy in 1..=3 {
            let below = BlockPos::new(base.x, base.y - dy, base.z);
            if let Some(state) = instance.get_block_state(below) {
                // Read the block's registry id (e.g. "soul_sand", "soul_soil")
                // rather than parsing a Debug string, so this stays correct
                // across block-state changes.
                let id = Box::<dyn BlockTrait>::from(state).id();
                if id.contains("soul") {
                    has_soul_sand = true;
                    break;
                }
            }
        }
        if has_soul_sand {
            out.push((base, e.name));
        }
    }
    out
}

// Auto-eat is driven by a timer in `behavior.rs` rather than the live hunger
// value: every few minutes the bot eats one food item if it has any. This is
// sufficient for an AFK companion.
