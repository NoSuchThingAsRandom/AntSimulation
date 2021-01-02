use crate::ant_settings::{
    DEFAULT_EXPLORATION_PHEROMONE_DEPRECIATION_RATE, DEFAULT_RESOURCE_PHEROMONE_DEPRECIATION_RATE,
    MAXIMUM_PHEROMONE_STRENGTH,
};
use enum_map::Enum;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Enum)]
/// The possible types a Pheromone can take
///
/// * Exploration - Used for Scout ants, to store the path they have take
/// * Resource - Used for marking the path to a resource
pub enum PheromoneType {
    Exploration,
    Resource,
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

/// This is a representation of a singular marker laid by ants
/// Should be updated every tick, and the strength reduces by the depreciation rate
#[derive(Copy, Clone)]
pub struct Pheromone {
    /// The current strength of the pheromone. Should be less than equal to the MAXIMUM_PHEROMONE_STRENGTH
    pub(crate) strength: u16,
    /// How much to reduce the strength by, per time step. Should be less than or equal to the strength
    depreciation_rate: u16,
    /// The type of Pheromone
    pub pheromone_type: PheromoneType,
}

impl Pheromone {
    /// Creates a new pheromone with the supplied arguments
    ///
    /// # Examples
    ///
    /// Creates a new pheromone instance
    /// ```
    /// use sim::pheromone::{Pheromone, PheromoneType};
    ///
    /// let strength: u16 = 50;
    /// let depreciation_rate = 1;
    /// let pheromone_type = PheromoneType::Exploration;
    /// let pheromone = Pheromone::new(strength, depreciation_rate, pheromone_type);
    /// # assert!(pheromone.is_some());
    /// ```    
    /// ```
    /// # // This will fail, as the depreciation rate, is greater than the initial strength
    /// # use sim::pheromone::{Pheromone, PheromoneType};
    /// # let strength: u16 = 50;
    /// # let pheromone_type = PheromoneType::Exploration;
    /// # assert!(Pheromone::new(strength, strength+1, pheromone_type).is_none())
    /// ```    
    /// ```
    /// # //This will fail as the strength, is greater than the MAXIMUM_PHEROMONE_STRENGTH
    /// # use sim::ant_settings::MAXIMUM_PHEROMONE_STRENGTH;
    /// # use sim::pheromone::{Pheromone, PheromoneType};
    ///
    /// # let strength: u16 = MAXIMUM_PHEROMONE_STRENGTH+1;
    /// # assert!(Pheromone::new(strength,1, PheromoneType::Resource).is_none())
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
    /// Creates a new default pheromone from the given type, with the strength and depreciation rate, defined in ant_settings
    pub fn default(pheromone_type: PheromoneType) -> Pheromone {
        let depreciation_rate = match pheromone_type {
            PheromoneType::Exploration => DEFAULT_EXPLORATION_PHEROMONE_DEPRECIATION_RATE,
            PheromoneType::Resource => DEFAULT_RESOURCE_PHEROMONE_DEPRECIATION_RATE,
        };
        Pheromone {
            strength: MAXIMUM_PHEROMONE_STRENGTH,
            depreciation_rate,
            pheromone_type,
        }
    }
    /// Creates a new default exploration pheromone, with the strength and depreciation rate, defined in ant_settings
    pub fn default_exploration() -> Pheromone {
        Pheromone {
            strength: MAXIMUM_PHEROMONE_STRENGTH,
            depreciation_rate: DEFAULT_EXPLORATION_PHEROMONE_DEPRECIATION_RATE,
            pheromone_type: PheromoneType::Exploration,
        }
    }
    /// Creates a new default resource pheromone, with the strength and depreciation rate, defined in ant_settings
    pub fn default_resource() -> Pheromone {
        Pheromone {
            strength: MAXIMUM_PHEROMONE_STRENGTH,
            depreciation_rate: DEFAULT_RESOURCE_PHEROMONE_DEPRECIATION_RATE,
            pheromone_type: PheromoneType::Resource,
        }
    }

    /// Used to increment the strength of a pheromone
    ///
    /// i.e. An ant walks over an existing pheromone and increases the strength by the given amount
    ///
    /// Will only increment, to the maximum pheromone strength
    /// # Examples:
    /// ```
    /// # use sim::pheromone::{Pheromone, PheromoneType};
    /// # use sim::ant_settings::MAXIMUM_PHEROMONE_STRENGTH;
    ///  let mut pheromone = Pheromone::new(10, 5, PheromoneType::Resource).unwrap();
    ///  assert_eq!(pheromone.get_strength(), 10);
    ///
    ///  pheromone.refresh(1);
    ///  assert_eq!(pheromone.get_strength(), 11);    
    ///
    ///  pheromone.refresh(MAXIMUM_PHEROMONE_STRENGTH);
    ///  assert_eq!(pheromone.get_strength(), MAXIMUM_PHEROMONE_STRENGTH);
    ///
    /// ```
    pub fn refresh(&mut self, strength: u16) {
        if let Some(strength) = self.strength.checked_add(strength) {
            if strength < MAXIMUM_PHEROMONE_STRENGTH {
                self.strength = strength;
                return;
            }
        }
        self.strength = MAXIMUM_PHEROMONE_STRENGTH;
    }

    /// Updates the strength of the pheromone for one time step (by reducing it by the depreciation rate)
    /// and returns true if the pheromone still exists (strength greater than 0)
    /// # Examples
    /// Creates a new pheromone and updates it every second, until it has deprecated to zero
    /// ```
    /// # use std::time::Duration;
    /// # use sim::pheromone::{Pheromone, PheromoneType};
    /// let mut pheromone = Pheromone::new(10,5, PheromoneType::Resource).unwrap();
    /// while pheromone.update(){
    ///     std::thread::sleep(Duration::from_secs(1));
    /// }
    /// # // Used in tests to check for underflow errors
    /// # pheromone.update();
    /// # pheromone.update();
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

    /// The Pheromones current strength
    pub fn get_strength(&self) -> u16 {
        self.strength
    }

    /*    /// Returns the color that the Pheromone should be rendered as
    ///
    /// The lightness depends on the strength of the Pheromone
    pub fn get_colour(&self) -> Color {
        let color = (200_f64 * ((self.get_strength() as f64) / (MAXIMUM_PHEROMONE_STRENGTH as f64)))
            as u8
            + 55;
        match self.pheromone_type {
            PheromoneType::Exploration => Color::from_rgb(color, 0, color),
            PheromoneType::Resource => Color::from_rgb(color, color, color),
        }
    }*/
}

impl Debug for Pheromone {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Display for Pheromone {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} has strength: {} and deprecates by: {}",
            self.pheromone_type, self.strength, self.depreciation_rate
        )
    }
}
