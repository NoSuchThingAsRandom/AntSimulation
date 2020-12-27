use crate::sim::world::World;

pub mod ant_settings;
pub mod sim;

pub fn main() {
    println!("Hello");
    let mut world = World::default();
    world.new_colony();
    for _ in 0..5 {
        world.update();
    }
}
