mod render;
use ant_lib::sim::Coordinates;
use ggez::{event, ContextBuilder};

fn main() {
    println!("Hello, world!");
    let test = Coordinates::new(15, 16).unwrap();
    test.modify(0, 1);
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Ant Simulation", "Sam")
        .build()
        .expect("Could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = render::Render::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
    ant_lib::main();
}
