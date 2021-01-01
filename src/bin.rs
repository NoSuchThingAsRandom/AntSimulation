mod ant_settings;
mod render;
mod sim;
use ggez::{event, ContextBuilder};

fn main() {
    // GGEZ Context
    let (mut ctx, mut event_loop) = ContextBuilder::new("Ant Simulation", "Sam")
        .build()
        .expect("Could not create ggez context!");

    // Instantiate a renderer for GGEZ
    let mut my_game = render::Render::new(&mut ctx);

    // Main Event loop
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occurred: {}", e),
    }
}
