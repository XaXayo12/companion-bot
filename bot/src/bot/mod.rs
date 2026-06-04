pub mod behavior;
pub mod handler;
pub mod identity;
pub mod tasks;
pub mod world_scan;

use azalea::Vec3;

pub fn dist(a: Vec3, b: Vec3) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}
