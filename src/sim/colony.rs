use crate::ant_settings::{DEBUG_MODE, DEFAULT_COLONY_SPAWN_RATE, WORLD_HEIGHT, WORLD_WIDTH};
use crate::sim::ant::{Ant, AntType};
use crate::sim::pheromone::{Pheromone, PheromoneType};
use crate::sim::resource::Resource;
use crate::sim::Coordinates;
use enum_map::EnumMap;
use std::collections::HashMap;

/// A container for a group of ants
///
/// Takes up one tile position
pub struct Colony {
    /// The coordinates of the colony position
    pub(crate) position: Coordinates,
    /// Stores all ants, by their type
    // TODO Switch to a faster map
    pub(crate) ants: HashMap<AntType, Vec<Ant>>,
    /// The maximum number of ants that can be spawned per time step
    spawn_rate: u16,
}

impl Default for Colony {
    fn default() -> Self {
        let mut ants = HashMap::new();
        ants.insert(AntType::Scout, Vec::new());
        ants.insert(AntType::Worker, Vec::new());
        Colony {
            position: Coordinates::new(WORLD_WIDTH / 2, WORLD_HEIGHT / 2).unwrap(),
            ants,
            spawn_rate: DEFAULT_COLONY_SPAWN_RATE,
        }
    }
}
impl Colony {
    /// Builds a new colony at the given position
    ///
    /// Ant types have to be added manually
    pub fn new(position: Coordinates) -> Colony {
        Colony {
            position,
            ants: HashMap::new(),
            spawn_rate: DEFAULT_COLONY_SPAWN_RATE,
        }
    }
    /// Spawns the maximum amount of ants that are allowed each turn
    ///
    /// Will evenly distribute the type of ants, by the amount of ants missing per type
    ///
    /// For example:
    ///     If the maximum number of ants per type is:
    ///         50 Scouts and 100 Workers
    ///     
    ///     And the colony currently has 10 Scouts and 50 Workers
    ///     Then:
    ///         50 - 10 = 40 Scouts are required
    ///         100 -50 = 50 Workers are required
    ///         40+50 = 90 is the total number of required ants
    ///
    ///     But say we can only spawn 20 ants per time step
    ///     Then:
    ///         40*(20/90) = 8.88 = 8 Scouts are spawned
    ///         50*(20/90) = 1.11 = 11 Workers are spawned
    ///
    fn spawn_ants(&mut self) {
        let mut total_required_ants = 0;
        let mut ants_spawn = Vec::new();

        // Counts the number of ants that are required, for each type
        for (ant_type, ants) in &self.ants {
            let max_ants = ant_type.get_maximum_number_of_ants() as u16;
            let required = max_ants - ants.len() as u16;
            if DEBUG_MODE {
                println!(
                    "Type: {} has maximum of {} and required: {}",
                    ant_type, max_ants, required
                );
            }
            if required > 0 {
                ants_spawn.push((*ant_type, required));
                total_required_ants += required as u16;
            }
        }
        if DEBUG_MODE {
            println!("Total ants to spawn: {}", total_required_ants);
            println!("Spawn rate: {}", self.spawn_rate);
        }
        // Allocates and spawns the number of ants that can be spawned this turn, between the number of ants that are required per type
        for (ant_type, amount) in ants_spawn {
            let mut to_spawn = amount * (self.spawn_rate * 100) / total_required_ants;
            to_spawn /= 100;
            if DEBUG_MODE {
                println!(
                    "Spawning: {} for type: {} at Position {} with required: {}",
                    to_spawn, ant_type, self.position, amount
                );
            }
            let ant_container = self
                .ants
                .get_mut(&ant_type)
                .unwrap_or_else(|| panic!("Failed to get ant type {}", ant_type));
            for _ in 0..to_spawn {
                ant_container.push(Ant::new(ant_type, self.position, self.position));
            }
        }
    }

    /// Spawns the maximum amount of ants it can for this time step
    ///
    /// And updates the position of all the ants in this colony
    pub fn update(
        &mut self,
        food_map: &mut [[Option<Resource>; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
        pheromones_lookup: &mut Vec<(Coordinates, PheromoneType)>,
        pheromones_map: &mut [[EnumMap<PheromoneType, Option<Pheromone>>; WORLD_HEIGHT as usize];
                 WORLD_WIDTH as usize],
    ) {
        self.spawn_ants();

        for (_, ants) in self.ants.iter_mut() {
            for ant in ants {
                ant.update(food_map, pheromones_lookup, pheromones_map);
            }
        }
    }

    /// Returns a copy of the colony position
    pub fn get_position(&self) -> Coordinates {
        self.position
    }

    /// Returns a iterator for all ants in the colony, contained by AntType
    pub fn iter_ants(&self) -> std::collections::hash_map::Iter<AntType, Vec<Ant>> {
        self.ants.iter()
    }
}
