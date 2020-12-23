use crate::ant_settings::{MAXIMUM_PHEROMONE_STRENGTH};
use crate::world::{AntType, World};
use ggez::event::EventHandler;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::{Color, DrawParam, Drawable, Image};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use std::time::{Duration, Instant};

/// This is the size of each individual tile in pixels
const TILE_SIZE: u16 = 16;
pub struct Render {
    world: World,
    game_ticks: usize,
    time_elapsed: Instant,
}
impl Render {
    pub fn new(_ctx: &mut Context) -> Render {
        let mut world = World::default();
        world.new_colony();
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
        if Instant::now().duration_since(self.time_elapsed) > Duration::from_secs(1) {
            self.game_ticks += 1;
            self.world.update();
            if let Some(time) = self.time_elapsed.checked_add(Duration::from_secs(1)) {
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
        for coords in &self.world.pheromone_lookup {
            if let Some(pheromone) =
                &self.world.pheromones[coords.x_position as usize][coords.y_position as usize]
            {
                sprite.add(DrawParam::src(
                    DrawParam::default()
                        .color(Color::from_rgb(
                            0,
                            (200_f64
                                * ((pheromone.strength as f64)
                                    / (MAXIMUM_PHEROMONE_STRENGTH as f64)))
                                as u8
                                + 55,
                            0,
                        ))
                        .dest(Point2::new(
                            TILE_SIZE as f32 * (coords.x_position) as f32,
                            TILE_SIZE as f32 * (coords.y_position) as f32,
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
        for colony in &self.world.colonies {
            for (ant_type, ants) in &colony.ants {
                for ant in ants {
                    sprite.add(DrawParam::src(
                        DrawParam::default()
                            .color(get_ant_type_colour(ant_type))
                            .dest(Point2::new(
                                TILE_SIZE as f32 * (ant.position.x_position) as f32,
                                TILE_SIZE as f32 * (ant.position.y_position) as f32,
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
                        TILE_SIZE as f32 * (colony.position.x_position) as f32,
                        TILE_SIZE as f32 * (colony.position.y_position) as f32,
                    )),
                graphics::Rect {
                    x: TILE_SIZE as f32,
                    y: TILE_SIZE as f32,
                    w: 1.0,
                    h: 1.0,
                },
            ));
        }

        sprite.draw(ctx, graphics::DrawParam::default())?;
        graphics::present(ctx)?;
        Ok(())
    }
}
pub fn get_ant_type_colour(ant_type: &AntType) -> Color {
    match ant_type {
        AntType::Scout => Color::from_rgb(0, 0, 255),
        AntType::Worker => Color::from_rgb(255, 255, 255),
    }
}
