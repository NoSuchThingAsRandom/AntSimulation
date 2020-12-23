use crate::world::World;

pub mod Render;
pub mod ant_settings;
pub mod world;

pub fn main() {
    println!("Hello");
    let mut world = World::default();
    world.new_colony();
    for _ in 0..5 {
        world.update();
    }
}
