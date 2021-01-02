use crate::ant_settings::{ANT_BACKWARDS_CHANCE, DEFAULT_COLONY_SCOUT_SIZE, DEFAULT_COLONY_WORKER_SIZE, DEFAULT_MAX_ANT_STEPS, DEFAULT_PHEROMONE_REFRESH_AMOUNT, DEFAULT_TERRITORY_SIZE, SCOUT_RETURN_PHEROMONE_CHANCE, WORKER_PHEROMONE_CHANCE, WORLD_HEIGHT, WORLD_WIDTH, DEBUG_MODE};

use crate::ant::AntType::Scout;
use enum_map::EnumMap;
use crate::pheromone::{Pheromone, PheromoneType};
use crate::resource::Resource;
use crate::Coordinates;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct Ant {
    ant_type: AntType,
    pub position: Coordinates,
    colony_position: Coordinates,
    steps_on_current_journey: u16,
    is_returning_to_colony: bool,
    found_food: bool,
    distance_from_colony: u16,
}

/// All possible directions that an ant can move in
const MOVE_POSSIBILITIES: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Ant {
    /// Creates a new ant, with the given type and position
    ///
    /// # Examples
    /// ```
    /// # use sim::ant_settings::WORLD_WIDTH;
    /// # use sim::ant::{Ant, AntType};
    /// # use sim::Coordinates;
    ///
    /// let colony_position = Coordinates::new(0, 5).unwrap();
    /// let position = Coordinates::new(0, 5).unwrap();
    /// let ant_type=AntType::Scout;
    ///
    /// let ant = Ant::new(ant_type, position, colony_position);
    /// ```
    pub fn new(ant_type: AntType, position: Coordinates, colony_position: Coordinates) -> Ant {
        Ant {
            ant_type,
            position,
            is_returning_to_colony: false,
            steps_on_current_journey: 0,
            colony_position,
            distance_from_colony: 0,
            found_food: false,
        }
    }
    /// Executes the next time step for this ant
    /// By:
    /// * Moving the ant
    /// * Updating any relevant pheromones
    /// * Consuming any available food
    pub fn update(
        &mut self,
        food_map: &mut [[Option<Resource>; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
        pheromones_lookup: &mut Vec<(Coordinates, PheromoneType)>,
        pheromones_map: &mut [[EnumMap<PheromoneType, Option<Pheromone>>; WORLD_HEIGHT as usize];
            WORLD_WIDTH as usize],
    ) {
        self.steps_on_current_journey += 1;
        // Consume food if it is available
        if let Some(mut food) =
        &food_map[self.position.x_position as usize][self.position.y_position as usize]
        {
            self.is_returning_to_colony = true;
            self.found_food = true;
            if food.consume().is_none() {
                food_map[self.position.x_position as usize][self.position.y_position as usize] =
                    None;
            }
        }
        self.move_ant(pheromones_map);
        self.update_pheromone(pheromones_lookup, pheromones_map);
    }

    /// If a pheromone of the correct type, already exists at the current position, then refreshes it
    ///
    /// Otherwise, creates a new default pheromone of the correct type at the current position
    fn update_pheromone(
        &self,
        pheromones_lookup: &mut Vec<(Coordinates, PheromoneType)>,
        pheromones_map: &mut [[EnumMap<PheromoneType, Option<Pheromone>>; WORLD_HEIGHT as usize];
            WORLD_WIDTH as usize],
    ) {
        // Determine the pheromone type
        let pheromone_type = if self.found_food {
            PheromoneType::Resource
        } else if self.ant_type == AntType::Scout && !self.is_returning_to_colony {
            PheromoneType::Exploration
        } else {
            return;
        };

        // Attempts to refresh the pheromone
        if let Some(pheromone) = &mut pheromones_map[self.position.x_position as usize]
            [self.position.y_position as usize][pheromone_type]
        {
            pheromone.refresh(DEFAULT_PHEROMONE_REFRESH_AMOUNT);
        } else {
            pheromones_map[self.position.x_position as usize][self.position.y_position as usize]
                [pheromone_type] = Some(Pheromone::default(pheromone_type));
            pheromones_lookup.push(((self.position), pheromone_type));
        }
    }

    /// Moves the ant, using one of the movement systems
    ///
    /// Is dependant on the ant type and probability of using a specified movement system, defined in [`ant_settings']
    fn move_ant(
        &mut self,
        pheromones_map: &[[EnumMap<PheromoneType, Option<Pheromone>>; WORLD_HEIGHT as usize];
            WORLD_WIDTH as usize],
    ) {
        // Reset if at the colony
        if self.position == self.colony_position {
            self.steps_on_current_journey = 0;
            self.is_returning_to_colony = false;
            self.found_food = false;
        }
        // If the journey has reached the max distance
        else if self.steps_on_current_journey > DEFAULT_MAX_ANT_STEPS {
            self.steps_on_current_journey = 0;
            self.is_returning_to_colony = true;
        }
        // The chance of an ant following the strongest pheromone trail
        let ant_pheromone_chance = match self.ant_type {
            AntType::Scout => {
                if self.is_returning_to_colony {
                    SCOUT_RETURN_PHEROMONE_CHANCE
                } else {
                    // Equation = y= 1/e^(distance/DEFAULT_TERRITORY_SIZE)
                    // Use the distance from colony, to influence the chance of taking established paths
                    // i.e. The further from the colony, the higher chance of moving randomly
                    1_f64 / (self.distance_from_colony as f64 / DEFAULT_TERRITORY_SIZE as f64).exp()
                }
            }
            AntType::Worker => WORKER_PHEROMONE_CHANCE,
        };

        // Apply the correct movement system
        let random_chance: f64 = rand::random();
        if random_chance < ant_pheromone_chance {
            self.move_using_pheromones(pheromones_map);
        } else {
            self.move_using_random();
        }
    }

    /// Checks if the new position is in the correct direction for the current ant status
    ///
    /// i.e:
    /// * If the ant is exploring or retrieving a resource, then checks if the new position is further from the colony
    /// * Or if the ant is returning to the colony, then checks if the new position is closer to the colony,
    fn is_correct_direction(&self, new_position: Coordinates) -> bool {
        let new_distance = new_position.manhattan_distance(self.colony_position);
        if self.is_returning_to_colony {
            new_distance < self.distance_from_colony
        } else {
            new_distance > self.distance_from_colony
        }
    }

    // TODO Causes sim to freeze when edge of world is reached, as it cannot find a valid move
    /// Moves the ant randomly in one of the possible directions given by: [`MOVE_POSSIBILITIES`]
    ///
    /// The chance of moving backwards, is defined in [`ant_settings']
    fn move_using_random(&mut self) {
        let mut allow_backwards = rand::random::<f64>() > ANT_BACKWARDS_CHANCE;
        let mut new_position = None;
        let mut moves = MOVE_POSSIBILITIES;
        moves.shuffle(&mut thread_rng());
        // Retrieves the first available valid move
        for new_move in &moves {
            // If a move exceeds the world boundaries, then allow backwards movement
            if let Some(test_position) = self.position.modify(new_move.0, new_move.1) {
                new_position = Some(test_position);
                if allow_backwards || self.is_correct_direction(test_position) {
                    break;
                }
            } else {
                allow_backwards = true;
            }
        }
        if new_position.is_none() {
            panic!(
                "Ant at {} cannot move, possible movements {:?}",
                self.position, moves
            );
        }
        // Apply the movement
        let new_position = new_position.unwrap();
        self.position = new_position;
        self.distance_from_colony = self.position.manhattan_distance(self.colony_position);
    }

    /// Moves the ant in the direction of the strongest valid pheromone     
    ///
    /// If there are no nearby valid pheromones then, moves in a random direction
    fn move_using_pheromones(
        &mut self,
        pheromones_map: &[[EnumMap<PheromoneType, Option<Pheromone>>; WORLD_HEIGHT as usize];
            WORLD_WIDTH as usize],
    ) {
        let mut strongest_pheromone = 0;
        let mut position = Coordinates::default();
        let mut moves = MOVE_POSSIBILITIES;
        moves.shuffle(&mut thread_rng());
        for move_possibility in &moves {
            let new_position = self
                .position
                .safe_modify(move_possibility.0, move_possibility.1);
            if !self.is_correct_direction(new_position) {
                continue;
            }

            let pheromones =
                &pheromones_map[new_position.x_position as usize][new_position.y_position as usize];

            if self.ant_type == Scout {
                if let Some(pheromone) = pheromones[PheromoneType::Exploration] {
                    if pheromone.strength > strongest_pheromone {
                        strongest_pheromone = pheromone.strength;
                        position = new_position;
                    }
                }
            }
            if let Some(pheromone) = &pheromones[PheromoneType::Resource] {
                if pheromone.strength > strongest_pheromone {
                    strongest_pheromone = pheromone.strength;
                    position = new_position;
                }
            }
        }
        // Fallback to random if no available pheromones
        if strongest_pheromone == 0 {
            self.move_using_random();
            return;
        }
        if self.found_food && DEBUG_MODE {
            println!(
                "Moving from {} to {} is_correct {} ",
                self.position,
                position,
                self.is_correct_direction(position)
            );
        }
        self.position = position;
        self.distance_from_colony = self.position.manhattan_distance(self.colony_position);
    }
}

/// The possible roles that an ant can take
/// * Scout - Will explore to try and find new resources
/// * Worker - Will move found resources to the colony
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum AntType {
    Scout,
    Worker,
}

impl AntType {
    /// Retrieves the maximum amount of ants each ant type can have from [`ant_settings`]
    pub(crate) fn get_maximum_number_of_ants(&self) -> u16 {
        match self {
            AntType::Scout => DEFAULT_COLONY_SCOUT_SIZE,
            AntType::Worker => DEFAULT_COLONY_WORKER_SIZE,
        }
    }
}

impl Display for AntType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AntType::Scout => write!(f, "Scout"),
            AntType::Worker => write!(f, "Worker"),
        }
    }
}
