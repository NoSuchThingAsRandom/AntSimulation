use crate::ant_settings::{
    ANT_BACKWARDS_CHANCE, DEFAULT_COLONY_SCOUT_SIZE, DEFAULT_COLONY_WORKER_SIZE,
    DEFAULT_MAX_ANT_STEPS, DEFAULT_TERRITORY_SIZE, PHEROMONE_TYPES_COUNT,
    SCOUT_RETURN_PHEROMONE_CHANCE, WORKER_PHEROMONE_CHANCE, WORLD_HEIGHT, WORLD_WIDTH,
};

use crate::sim::ant::AntType::Scout;
use crate::sim::pheromone::{Pheromone, PheromoneType};
use crate::sim::resource::Resource;
use crate::sim::Coordinates;
use ggez::graphics::Color;
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

const MOVE_POSSIBILITIES: [(i16, i16); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Ant {
    /// Creates a new ant, with the given type and position, if it is inside the world boundary
    ///
    /// # Examples
    /// ```
    /// # use ant_lib::ant_settings::WORLD_WIDTH;
    /// # use ant_lib::world::{Ant, AntType, Coordinates};
    /// use ant_lib::sim::Coordinates;
    ///
    /// let position = Coordinates::new(0,5).unwrap();
    /// let ant_type=AntType::Scout;
    ///
    /// let ant = Ant::new(ant_type,position);
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
    /// This will:
    /// * Move the ant
    /// * Update any relevant pheromones
    /// * Consume any available food
    pub fn update(
        &mut self,
        food_map: &mut [[Option<Resource>; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
        pheromones_lookup: &mut Vec<(Coordinates, PheromoneType)>,
        pheromones_map: &mut [[[Option<Pheromone>; PHEROMONE_TYPES_COUNT]; WORLD_HEIGHT as usize];
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

    /// If a pheromone of the correct type, already exists at the current position, then reinforces it
    ///
    /// Otherwise creates a new default pheromone of the correct type at the current position
    fn update_pheromone(
        &self,
        pheromones_lookup: &mut Vec<(Coordinates, PheromoneType)>,
        pheromones_map: &mut [[[Option<Pheromone>; PHEROMONE_TYPES_COUNT]; WORLD_HEIGHT as usize];
                 WORLD_WIDTH as usize],
    ) {
        let pheromone_type = if self.found_food {
            PheromoneType::Resource
        } else if self.ant_type == AntType::Scout && !self.is_returning_to_colony {
            PheromoneType::Exploration
        } else {
            return;
        };

        // Attempts to reinforce the pheromone
        if let Some(pheromone) = &mut pheromones_map[self.position.x_position as usize]
            [self.position.y_position as usize][pheromone_type.as_pheromone_index()]
        {
            pheromone.refresh(pheromone.strength);
        } else {
            pheromones_map[self.position.x_position as usize][self.position.y_position as usize]
                [pheromone_type.as_pheromone_index()] = Some(Pheromone::default(pheromone_type));
            pheromones_lookup.push(((self.position), pheromone_type));
        }
    }

    /// Moves the ant, using one of the movement systems, dependant on the ant type and probability
    ///
    /// Ant Scout:
    ///     25% Chance of following strongest pheromone
    ///     75% Chance of randomly moving
    ///
    /// Ant Worker:
    ///     75% Chance of following strongest pheromone
    ///     25% Chance of randomly moving
    fn move_ant(
        &mut self,
        pheromones_map: &[[[Option<Pheromone>; PHEROMONE_TYPES_COUNT]; WORLD_HEIGHT as usize];
             WORLD_WIDTH as usize],
    ) {
        if self.position == self.colony_position {
            self.steps_on_current_journey = 0;
            self.is_returning_to_colony = false;
            self.found_food = false;
        } else if self.steps_on_current_journey > DEFAULT_MAX_ANT_STEPS {
            self.steps_on_current_journey = 0;
            self.is_returning_to_colony = true;
        }
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

        // TODO Use pheromones to influence ant direction
        let random_chance: f64 = rand::random();
        if random_chance < ant_pheromone_chance {
            self.move_pheromones(pheromones_map);
        } else {
            self.move_random();
        }
    }

    /// Checks if the new position is closer/further to the colony, depending on whether the ant is moving away/to the colony
    ///
    /// Basically if, the ant is_returning_to_colony, then returns true if the new position is closer to the colony
    /// Else returns true if the new position is further away from the colony
    fn is_correct_direction(&self, new_position: Coordinates) -> bool {
        let new_distance = new_position.manhattan_distance(self.colony_position);
        if self.is_returning_to_colony {
            new_distance < self.distance_from_colony
        } else {
            new_distance > self.distance_from_colony
        }
    }

    // TODO Causes sim to freeze when edge of world is reached
    /// Moves the ant randomly in one of the possible directions given by: MOVE_POSSIBILITIES
    fn move_random(&mut self) {
        let mut allow_backwards = rand::random::<f64>() > ANT_BACKWARDS_CHANCE;
        let mut new_position = None;
        let mut moves = MOVE_POSSIBILITIES.clone();
        moves.shuffle(&mut thread_rng());
        for new_move in &moves {
            if let Some(test_position) = self.position.modify(new_move.0, new_move.1) {
                new_position = Some(test_position);
                if allow_backwards || self.is_correct_direction(test_position) {
                    break;
                }
            } else {
                allow_backwards = true;
            }
        }
        // Should be a possible valid move
        if new_position.is_none() {
            panic!(
                "Ant at {} cannot move, selection {:?}",
                self.position, moves
            );
        }
        let new_position = new_position.unwrap();
        self.position = new_position;
        self.distance_from_colony = self.position.manhattan_distance(self.colony_position);
    }

    /// Moves the ant in the direction of the strongest pheromone (of the possible directions given by: MOVE_POSSIBILITIES)
    ///
    /// If there are no nearby pheromones then, moves in a random direction
    fn move_pheromones(
        &mut self,
        pheromones_map: &[[[Option<Pheromone>; PHEROMONE_TYPES_COUNT]; WORLD_HEIGHT as usize];
             WORLD_WIDTH as usize],
    ) {
        let mut strongest_pheromone = 0;
        let mut position = Coordinates::default();
        let mut moves = MOVE_POSSIBILITIES.clone();
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
                if let Some(pheromone) = pheromones[PheromoneType::Exploration.as_pheromone_index()]
                {
                    if pheromone.strength > strongest_pheromone {
                        strongest_pheromone = pheromone.strength;
                        position = new_position;
                    }
                }
            }
            if let Some(pheromone) = &pheromones[PheromoneType::Resource.as_pheromone_index()] {
                if pheromone.strength > strongest_pheromone {
                    strongest_pheromone = pheromone.strength;
                    position = new_position;
                }
            }
        }
        // Fallback to random if no available pheromones
        if strongest_pheromone == 0 {
            self.move_random();
            return;
        }
        if self.found_food {
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

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum AntType {
    Scout,
    Worker,
}

impl AntType {
    pub(crate) fn get_maximum_number_of_ants(&self) -> u16 {
        match self {
            AntType::Scout => DEFAULT_COLONY_SCOUT_SIZE,
            AntType::Worker => DEFAULT_COLONY_WORKER_SIZE,
        }
    }

    /// Returns the colour to render the Ant Type as
    pub fn get_render_color(&self) -> Color {
        match self {
            AntType::Scout => Color::from_rgb(0, 0, 255),
            AntType::Worker => Color::from_rgb(50, 190, 190),
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
