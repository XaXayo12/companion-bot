//! Human-like timing and aim, so the bot does not act with frame-perfect,
//! robotic precision. Every delay and offset here is randomized within a small
//! band, the way a real player's reactions vary.

use std::cell::RefCell;
use std::time::Duration;

use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_entropy());
}

fn with_rng<T>(f: impl FnOnce(&mut SmallRng) -> T) -> T {
    RNG.with(|cell| f(&mut cell.borrow_mut()))
}

/// A random delay in `[min_ms, max_ms]` for pacing async actions like opening a
/// chest or moving an item.
pub fn delay(min_ms: u64, max_ms: u64) -> Duration {
    let hi = max_ms.max(min_ms);
    Duration::from_millis(with_rng(|r| r.gen_range(min_ms..=hi)))
}

/// Sleep a random human-like amount (async).
pub async fn pause(min_ms: u64, max_ms: u64) {
    tokio::time::sleep(delay(min_ms, max_ms)).await;
}

/// A small random aim offset (in block units) so the bot never looks
/// pixel-perfectly at the exact centre of a target.
pub fn aim_jitter() -> f64 {
    with_rng(|r| r.gen_range(-0.12..0.12))
}

/// True with probability `p` (0.0..1.0). Used to make periodic actions slightly
/// irregular instead of firing on an exact clock.
pub fn chance(p: f64) -> bool {
    with_rng(|r| r.gen_range(0.0..1.0) < p)
}

/// A random number of ticks in `[min, max]`, e.g. a reaction delay added on top
/// of a base cooldown.
pub fn ticks(min: u32, max: u32) -> u32 {
    let hi = max.max(min);
    with_rng(|r| r.gen_range(min..=hi))
}

/// A small random head turn, in degrees, for a subtle anti-AFK glance. Never
/// large enough to look like a spin.
pub fn turn_degrees() -> f32 {
    with_rng(|r| r.gen_range(-25.0..25.0))
}
