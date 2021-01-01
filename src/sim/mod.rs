use crate::ant_settings::{WORLD_HEIGHT, WORLD_WIDTH};

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub mod ant;
pub mod colony;
pub mod pheromone;
pub mod resource;
pub mod world;

pub fn trim_f64(value: f64) -> u32 {
    (value * 1000_f64) as u32
}

/// Used for referencing the location of a tile in the world
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
    /// Attempts to create a new Coordinate with the given position, provided it is within the World boundaries
    /// # Examples
    /// Inside the world boundaries
    /// ```
    /// # use Ants::sim::Coordinates;
    ///
    /// let position = Coordinates::new(5, 5);
    /// assert!(position.is_some());
    ///
    /// let position = position.unwrap();
    /// assert_eq!(position.get_x_position_u16(), 5);
    /// assert_eq!(position.get_y_position_u16(), 5);
    /// ```    
    ///
    /// When exceeding the world boundaries
    /// ```
    /// # use Ants::sim::Coordinates;
    /// # use Ants::ant_settings::{WORLD_WIDTH,WORLD_HEIGHT};
    ///
    /// let position = Coordinates::new(WORLD_WIDTH + 1, WORLD_HEIGHT + 1);
    /// assert!(position.is_none());
    /// ```
    pub fn new(x_position: u16, y_position: u16) -> Option<Coordinates> {
        if x_position > WORLD_WIDTH || y_position > WORLD_HEIGHT {
            return None;
        }
        Some(Coordinates {
            x_position,
            y_position,
        })
    }

    /// Creates a new random coordinate, inside the world boundaries
    /// # Example
    /// ```
    /// # use Ants::sim::Coordinates;
    /// # use Ants::ant_settings::{WORLD_WIDTH,WORLD_HEIGHT};
    ///
    /// let new_position = Coordinates::new_random();
    ///
    /// assert!(new_position.get_x_position_u16() <= WORLD_WIDTH);
    /// assert!(new_position.get_y_position_u16() <= WORLD_HEIGHT);
    /// ```
    pub fn new_random() -> Coordinates {
        let x_position: u16 = ((rand::random::<f64>()) * (WORLD_WIDTH as f64)) as u16;
        let y_position: u16 = ((rand::random::<f64>()) * (WORLD_HEIGHT as f64)) as u16;
        Coordinates {
            x_position,
            y_position,
        }
    }
    /// Returns a copy of the current position, adjusted by the given amount, whilst staying in the world boundaries:
    /// # Examples
    /// Inside the world boundaries
    /// ```
    /// # use Ants::sim::Coordinates;
    ///
    /// let position = Coordinates::new(5, 5).unwrap();
    /// let new_position = position.safe_modify(-2, 7);
    ///
    /// assert_eq!(new_position.get_x_position_u16(), 3);
    /// assert_eq!(new_position.get_y_position_u16(), 12);
    /// ```    
    ///
    /// When exceeding the world boundaries
    /// ```
    /// # use Ants::sim::Coordinates;
    /// # use Ants::ant_settings::{WORLD_WIDTH,WORLD_HEIGHT};
    ///
    /// let position = Coordinates::new(WORLD_WIDTH, WORLD_HEIGHT).unwrap();
    /// let new_position = position.safe_modify(1, 1);
    ///
    /// assert_eq!(new_position.get_x_position_u16(), WORLD_WIDTH);
    /// assert_eq!(new_position.get_y_position_u16(), WORLD_HEIGHT);
    /// ```
    ///
    /// When less than the world boundaries
    /// ```    
    /// # use Ants::sim::Coordinates;
    ///
    /// let position = Coordinates::new(0, 0).unwrap();
    /// let new_position = position.safe_modify(-1, -1);
    ///
    /// assert_eq!(new_position.get_x_position_u16(), 0);
    /// assert_eq!(new_position.get_y_position_u16(), 0);
    /// ```
    pub fn safe_modify(&self, x_amount: i32, y_amount: i32) -> Coordinates {
        let mut output = Coordinates::default();

        let new_position = (self.x_position as i32)
            .checked_add(x_amount)
            .unwrap_or(WORLD_WIDTH as i32);
        output.x_position = if new_position > WORLD_WIDTH as i32 {
            WORLD_WIDTH
        } else if new_position < 0 {
            0
        } else {
            new_position as u16
        };
        let new_position = (self.y_position as i32)
            .checked_add(y_amount)
            .unwrap_or(WORLD_HEIGHT as i32);
        output.y_position = if new_position > WORLD_HEIGHT as i32 {
            WORLD_HEIGHT
        } else if new_position < 0 {
            0
        } else {
            new_position as u16
        };
        output
    }

    /// Attempts to add the amount given to a copy of the current position
    ///
    /// Returning None, if it would exceed the world boundaries
    /// # Examples
    /// Inside the world boundaries
    /// ```
    /// # use Ants::sim::Coordinates;
    ///
    /// let position = Coordinates::new(5, 5).unwrap();
    /// let new_position = position.modify(-2, 7);
    ///
    /// assert!(new_position.is_some());
    /// ```    
    ///
    /// When exceeding the world boundaries
    /// ```
    /// # use Ants::sim::Coordinates;
    /// # use Ants::ant_settings::{WORLD_WIDTH,WORLD_HEIGHT};
    ///
    /// let position = Coordinates::new(WORLD_WIDTH, WORLD_HEIGHT).unwrap();
    /// let new_position = position.modify(1, 1);
    ///
    /// assert!(new_position.is_none());
    /// ```
    ///
    /// When less than the world boundaries
    /// ```    
    /// # use Ants::sim::Coordinates;
    ///
    /// let position = Coordinates::new(0, 0).unwrap();
    /// let new_position = position.modify(-1, -1);
    ///
    /// assert!(new_position.is_none());
    /// ```
    pub fn modify(&self, x_amount: i32, y_amount: i32) -> Option<Coordinates> {
        let mut output = Coordinates::default();

        let new_position = (self.x_position as i32).checked_add(x_amount)?;
        output.x_position = if new_position >= WORLD_WIDTH as i32 {
            return None;
        } else if new_position < 0 {
            return None;
        } else {
            new_position as u16
        };
        let new_position = (self.y_position as i32).checked_add(y_amount)?;
        output.y_position = if new_position >= WORLD_HEIGHT as i32 {
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

    /// Computes the Manhattan distance, between this and the given coordinates
    /// # Example
    /// ```
    /// # use Ants::sim::Coordinates;
    ///
    /// let position = Coordinates::new(5,5).unwrap();
    /// let other = Coordinates::new(7,7).unwrap();
    ///
    /// assert_eq!(position.manhattan_distance(other), 4);
    ///
    /// ```
    pub fn manhattan_distance(&self, other: Coordinates) -> u16 {
        let x_distance = (self.x_position as i32 - other.x_position as i32).abs() as u16;
        let y_distance = (self.x_position as i32 - other.x_position as i32).abs() as u16;
        x_distance + y_distance
    }
}
