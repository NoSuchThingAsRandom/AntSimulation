use crate::ant_settings::{
    DEFAULT_EXPLORATION_PHEROMONE_DEPRECIATION_RATE, DEFAULT_FOOD_PHEROMONE_DEPRECIATION_RATE,
    MAXIMUM_PHEROMONE_STRENGTH, PHEROMONE_TYPES_COUNT,
};
use ggez::graphics::Color;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Pointer};

/// This is a representation of a singular marker laid by ants
/// Should be updated every tick, and the strength reduces by the depreciation rate
#[derive(Copy, Clone)]
pub struct Pheromone {
    /// The current strength of the pheromone. Should be less than equal to the MAXIMUM_PHEROMONE_STRENGTH
    pub(crate) strength: u16,
    /// How much to reduce the strength by, per time step. Should be less than or equal to the strength
    depreciation_rate: u16,
    pub(crate) pheromone_type: PheromoneType,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PheromoneType {
    Exploration,
    Resource,
}
impl PheromoneType {
    // TODO Need a better way of doing this
    /// Returns the index of each Pheromone, in the Pheromone data store
    pub fn as_pheromone_index(&self) -> usize {
        assert_eq!(2, PHEROMONE_TYPES_COUNT);
        match self {
            PheromoneType::Resource => 0,
            PheromoneType::Exploration => 1,
        }
    }
}
impl Debug for PheromoneType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
impl Display for PheromoneType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PheromoneType::Exploration => write!(f, "Exploration"),
            PheromoneType::Resource => write!(f, "Resource"),
        }
    }
}
// Removed due to Pheromone Type
/*impl Default for Pheromone {
    fn default() -> Self {
        Pheromone {
            strength: MAXIMUM_PHEROMONE_STRENGTH,
            depreciation_rate: 2,
            pheromone_type: PheromoneType::Exploration,
        }
    }
}*/

impl Pheromone {
    /// Creates a new pheromone with the supplied arguments
    ///
    /// # Examples
    ///
    /// Creates a new pheromone instance
    /// ```
    /// use ant_lib::world::Pheromone;
    ///
    /// let strength:u8=50;
    /// let depreciation_rate=1;
    /// let pheromone=Pheromone::new(strength,depreciation_rate);
    /// # assert!(pheromone.is_some());
    /// ```    
    /// ```
    /// # // This will fail, as the depreciation rate, is greater than the initial strength
    /// # let strength:u8=50;
    /// # assert!(ant_lib::world::Pheromone::new(strength,strength+1).is_none())
    /// ```    
    /// ```
    /// # //This will fail as the strength, is greater than the MAXIMUM_PHEROMONE_STRENGTH
    /// # use ant_lib::ant_settings::MAXIMUM_PHEROMONE_STRENGTH;
    /// # let strength:u16=MAXIMUM_PHEROMONE_STRENGTH+1;
    /// # assert!(ant_lib::world::Pheromone::new(strength,1).is_none())
    /// ```
    pub fn new(
        strength: u16,
        depreciation_rate: u16,
        pheromone_type: PheromoneType,
    ) -> Option<Pheromone> {
        if MAXIMUM_PHEROMONE_STRENGTH < strength || strength < depreciation_rate {
            return None;
        }
        Some(Pheromone {
            strength,
            depreciation_rate,
            pheromone_type,
        })
    }
    pub fn default(pheromone_type: PheromoneType) -> Pheromone {
        let depreciation_rate = match pheromone_type {
            PheromoneType::Exploration => DEFAULT_EXPLORATION_PHEROMONE_DEPRECIATION_RATE,
            PheromoneType::Resource => DEFAULT_FOOD_PHEROMONE_DEPRECIATION_RATE,
        };
        Pheromone {
            strength: MAXIMUM_PHEROMONE_STRENGTH,
            depreciation_rate,
            pheromone_type,
        }
    }

    pub fn default_exploration() -> Pheromone {
        Pheromone {
            strength: MAXIMUM_PHEROMONE_STRENGTH,
            depreciation_rate: DEFAULT_EXPLORATION_PHEROMONE_DEPRECIATION_RATE,
            pheromone_type: PheromoneType::Exploration,
        }
    }

    pub fn default_food() -> Pheromone {
        Pheromone {
            strength: MAXIMUM_PHEROMONE_STRENGTH,
            depreciation_rate: DEFAULT_FOOD_PHEROMONE_DEPRECIATION_RATE,
            pheromone_type: PheromoneType::Resource,
        }
    }

    /// Used to increment the strength of a pheromone
    ///
    /// i.e. An ant walks over an existing pheromone and increases the strength by the given amount
    pub fn refresh(&mut self, strength: u16) {
        if let Some(strength) = self.strength.checked_add(strength) {
            if strength < MAXIMUM_PHEROMONE_STRENGTH {
                self.strength = strength;
                return;
            }
        }
        self.strength = MAXIMUM_PHEROMONE_STRENGTH - 1;
    }

    /// Updates the strength of the pheromone for one time step (by reducing it by the depreciation rate)
    /// and returns true if the pheromone still exists (strength greater than 0)
    /// # Examples
    /// Creates a new pheromone and updates it every second, until it has depreceated to zero
    /// ```
    /// use std::time::Duration;
    /// let mut pheromone = ant_lib::world::Pheromone::new(10,5).unwrap();
    /// while pheromone.update(){
    ///     std::thread::sleep(Duration::from_secs(1));
    /// }
    /// # // Used in tests to check for underflow errors
    /// # pheromone.update();
    /// # pheromone.update();
    /// // Pheromone, should now be deleted
    /// ```
    pub fn update(&mut self) -> bool {
        if let Some(strength) = self.strength.checked_sub(self.depreciation_rate) {
            self.strength = strength;
            true
        } else {
            self.strength = 0;
            false
        }
    }

    /// Returns a copy of the Pheromones current strength
    pub fn get_strength(&self) -> u16 {
        self.strength
    }

    pub fn get_colour(&self) -> Color {
        let color = (200_f64 * ((self.get_strength() as f64) / (MAXIMUM_PHEROMONE_STRENGTH as f64)))
            as u8
            + 55;
        match self.pheromone_type {
            PheromoneType::Exploration => Color::from_rgb(color, 0, color),
            PheromoneType::Resource => Color::from_rgb(color, color, color),
        }
    }
}
