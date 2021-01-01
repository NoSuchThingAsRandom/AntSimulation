use crate::ant_settings::{
    DEBUG_MODE, DEFAULT_RESOURCE_SIZE, MAXIMUM_PHEROMONE_STRENGTH, WORLD_HEIGHT, WORLD_WIDTH,
};
use crate::sim::world::World;
use ggez::event::EventHandler;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::{Color, DrawParam, Drawable, Image};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use std::time::{Duration, Instant};

/// This is the size of each individual tile in pixels
const TILE_SIZE: u16 = 8;
pub struct Render {
    world: World,
    game_ticks: usize,
    time_elapsed: Instant,
}
impl Render {
    pub fn new(_ctx: &mut Context) -> Render {
        let mut world = World::default();
        Render {
            world,
            game_ticks: 0,
            time_elapsed: Instant::now(),
        }
    }
}
impl EventHandler for Render {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Does a game tick every second
        if Instant::now().duration_since(self.time_elapsed) > Duration::from_millis(250) {
            println!("\n\n-----\nTick {}\n----", self.game_ticks);
            if DEBUG_MODE {}
            self.game_ticks += 1;
            self.world.update();
            if let Some(time) = self.time_elapsed.checked_add(Duration::from_millis(250)) {
                self.time_elapsed = time;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let img: Image = Image::from_rgba8(
            ctx,
            (TILE_SIZE) as u16,
            (TILE_SIZE) as u16,
            &vec![255; 4 * TILE_SIZE as usize * TILE_SIZE as usize],
        )?;
        let mut sprite = SpriteBatch::new(img);
        // Draw Borders
        /*        sprite.add(DrawParam::src(
            DrawParam::default()
                .color(Color::from_rgb(128, 128, 128))
                .dest(Point2::new(0_f32, 0_f32)),
            graphics::Rect {
                x: 0_f32,
                y: 0_f32,
                w: WORLD_WIDTH as f32,
                h: WORLD_HEIGHT as f32,
            },
        ));*/
        // Draw Pheromones
        for (coords, pheromone_type) in &self.world.pheromone_lookup {
            if let Some(pheromone) = &self.world.pheromones[coords.get_x_position_usize()]
                [coords.get_y_position_usize()][*pheromone_type]
            {
                sprite.add(DrawParam::src(
                    DrawParam::default()
                        .color(pheromone.get_colour())
                        .dest(Point2::new(
                            TILE_SIZE as f32 * (coords.get_x_position_u16()) as f32,
                            TILE_SIZE as f32 * (coords.get_y_position_u16()) as f32,
                        )),
                    graphics::Rect {
                        x: TILE_SIZE as f32,
                        y: TILE_SIZE as f32,
                        w: 1.0,
                        h: 1.0,
                    },
                ));
            }
        }
        // Draw Ants
        for colony in &self.world.colonies {
            for (ant_type, ants) in colony.iter_ants() {
                let colour = ant_type.get_render_color();
                for ant in ants {
                    sprite.add(DrawParam::src(
                        DrawParam::default().color(colour).dest(Point2::new(
                            TILE_SIZE as f32 * (ant.position.get_x_position_u16()) as f32,
                            TILE_SIZE as f32 * (ant.position.get_y_position_u16()) as f32,
                        )),
                        graphics::Rect {
                            x: TILE_SIZE as f32,
                            y: TILE_SIZE as f32,
                            w: 1.0,
                            h: 1.0,
                        },
                    ));
                }
            }
            sprite.add(DrawParam::src(
                DrawParam::default()
                    .color(Color::from_rgb(255, 0, 0))
                    .dest(Point2::new(
                        TILE_SIZE as f32 * (colony.get_position().get_y_position_u16()) as f32,
                        TILE_SIZE as f32 * (colony.get_position().get_y_position_u16()) as f32,
                    )),
                graphics::Rect {
                    x: TILE_SIZE as f32,
                    y: TILE_SIZE as f32,
                    w: 1.0,
                    h: 1.0,
                },
            ));
        }
        // Draw Resources
        for coords in &self.world.resource_lookup {
            if let Some(resource) =
                &self.world.resources[coords.get_x_position_usize()][coords.get_y_position_usize()]
            {
                sprite.add(DrawParam::src(
                    DrawParam::default()
                        .color(Color::from_rgb(
                            220, 220,
                            40, /*                            0,
                               (200_f64
                                   * ((resource.get_percentage_remaining())
                                       / (DEFAULT_RESOURCE_SIZE as f64)))
                                   as u8
                                   + 55,
                               (200_f64
                                   * ((resource.get_percentage_remaining())
                                       / (DEFAULT_RESOURCE_SIZE as f64)))
                                   as u8
                                   + 55,*/
                        ))
                        .dest(Point2::new(
                            TILE_SIZE as f32 * (coords.get_x_position_u16()) as f32,
                            TILE_SIZE as f32 * (coords.get_y_position_u16()) as f32,
                        )),
                    graphics::Rect {
                        x: TILE_SIZE as f32,
                        y: TILE_SIZE as f32,
                        w: 1.0,
                        h: 1.0,
                    },
                ));
            }
        }

        sprite.draw(ctx, graphics::DrawParam::default())?;
        graphics::present(ctx)?;
        Ok(())
    }
}
