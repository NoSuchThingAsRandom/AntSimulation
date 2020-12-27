use crate::ant_settings::{WORLD_HEIGHT, WORLD_WIDTH};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

mod ant;
mod colony;
mod pheromone;
mod resource;
pub mod world;

pub fn trim_f64(value: f64) -> u32 {
    (value * 1000_f64) as u32
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coordinates {
    x_position: u16,
    y_position: u16,
}
impl Default for Coordinates {
    fn default() -> Self {
        Coordinates {
            x_position: 0,
            y_position: 0,
        }
    }
}
impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x_position, self.y_position)
    }
}
impl Debug for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
impl Coordinates {
    pub fn new(x_position: u16, y_position: u16) -> Option<Coordinates> {
        if x_position > WORLD_WIDTH || y_position > WORLD_HEIGHT {
            return None;
        }
        Some(Coordinates {
            x_position,
            y_position,
        })
    }

    /// Creates new random coordinates, inside the world boundaries
    pub fn new_random() -> Coordinates {
        let x_position: u16 = ((rand::random::<f64>()) * (WORLD_WIDTH as f64)) as u16;
        let y_position: u16 = ((rand::random::<f64>()) * (WORLD_HEIGHT as f64)) as u16;
        Coordinates {
            x_position,
            y_position,
        }
    }
    /// Will return the new ant coordinates after adjusting by the given amount, whilst staying in the world boundaries:
    /// (0..WORLD_WIDTH),(0..WORLD_HEIGHT)
    ///
    /// # Returns
    /// The new (x_position, y_position)
    pub fn safe_modify(&self, x_amount: i16, y_amount: i16) -> Coordinates {
        let mut output = Coordinates::default();

        let new_position = (self.x_position as i16)
            .checked_add(x_amount)
            .unwrap_or(WORLD_WIDTH as i16 - 1);
        output.x_position = if new_position >= WORLD_WIDTH as i16 {
            WORLD_WIDTH - 1
        } else if new_position < 0 {
            0
        } else {
            new_position as u16
        };
        let new_position = (self.y_position as i16)
            .checked_add(y_amount)
            .unwrap_or(WORLD_HEIGHT as i16 - 1);
        output.y_position = if new_position >= WORLD_HEIGHT as i16 {
            WORLD_HEIGHT - 1
        } else if new_position < 0 {
            0
        } else {
            new_position as u16
        };
        output
    }
    pub fn modify(&self, x_amount: i16, y_amount: i16) -> Option<Coordinates> {
        let mut output = Coordinates::default();

        let new_position = (self.x_position as i16).checked_add(x_amount)?;
        output.x_position = if new_position >= WORLD_WIDTH as i16 {
            return None;
        } else if new_position < 0 {
            return None;
        } else {
            new_position as u16
        };
        let new_position = (self.y_position as i16).checked_add(y_amount)?;
        output.y_position = if new_position >= WORLD_HEIGHT as i16 {
            return None;
        } else if new_position < 0 {
            return None;
        } else {
            new_position as u16
        };
        Some(output)
    }

    pub fn get_x_position_u16(&self) -> u16 {
        self.x_position
    }
    pub fn get_x_position_usize(&self) -> usize {
        self.x_position as usize
    }
    pub fn get_y_position_u16(&self) -> u16 {
        self.y_position
    }
    pub fn get_y_position_usize(&self) -> usize {
        self.y_position as usize
    }

    /// Computes the manhatten distance, between this and other
    pub fn manhattan_distance(&self, other: Coordinates) -> u16 {
        let x_distance = (self.x_position as i32 - other.x_position as i32).abs() as u16;
        let y_distance = (self.x_position as i32 - other.x_position as i32).abs() as u16;
        x_distance + y_distance
    }
}
