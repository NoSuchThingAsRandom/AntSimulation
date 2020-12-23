use crate::ant_settings::{
    DEBUG_MODE, DEFAULT_COLONY_SCOUT_SIZE, DEFAULT_COLONY_SPAWN_RATE, DEFAULT_COLONY_WORKER_SIZE,
    DEFAULT_RESOURCE_SIZE, MAXIMUM_PHEROMONE_STRENGTH, WORLD_HEIGHT, WORLD_WIDTH,
};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

/// A struct containing every entity in the world
///
/// All entities/objects are accessed through this
pub struct World {
    /// A container all active food objects
    pub food: [[Option<Food>; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
    pub food_lookup: Vec<Coordinates>,
    /// A container for all active colonies
    pub colonies: Vec<Colony>,
    /// A container for all active pheromones (with their x/y positions)
    pub pheromones: [[Option<Pheromone>; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
    pub pheromone_lookup: Vec<Coordinates>,
}
impl Default for World {
    fn default() -> Self {
        World {
            food: [[None; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
            food_lookup: Vec::new(),
            colonies: vec![],
            pheromones: [[None; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
            pheromone_lookup: Vec::new(),
        }
    }
}

impl World {
    /// Creates a new World instance with the supplied arguments
    ///
    /// # Arguments
    /// * `food*` A vector with all food instances that should exist on creation
    /// * `colonies*` A vector with all colonies instances that should exist on creation
    ///
    pub fn new(food: Vec<(Coordinates, Food)>, colonies: Vec<Colony>) -> World {
        let mut food_container = [[None; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize];
        let mut food_lookup = Vec::new();
        for (coords, food_entry) in food {
            food_container[coords.x_position as usize][coords.y_position as usize] =
                Some(food_entry);
            food_lookup.push(coords);
        }
        World {
            food: food_container,
            food_lookup,
            colonies,
            pheromones: [[None; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
            pheromone_lookup: Vec::new(),
        }
    }
    /// Creates a new default colony, and adds it to the world
    pub fn new_colony(&mut self) {
        self.colonies.push(Colony::default());
    }

    /// The main updater method
    /// This will:
    /// * Spawn any new food/ants if required
    /// * Update the position of ants
    /// * Update the strength of pheromones and remove them if necessary
    pub fn update(&mut self) {
        for colony in &mut self.colonies {
            colony.update(
                &mut self.food,
                &mut self.pheromone_lookup,
                &mut self.pheromones,
            );
        }
        let mut test = self.pheromone_lookup.clone();
        test.retain(|coords| {
            let mut retain = true;
            if let Some(pheromone) =
                &mut self.pheromones[coords.x_position as usize][coords.y_position as usize]
            {
                if !pheromone.update() {
                    retain = false;
                }
            }
            if !retain {
                self.pheromones[coords.x_position as usize][coords.y_position as usize] = None;
            }
            retain
        });
        self.pheromone_lookup = test;
        //self.display();
    }

    /// Prints a grid of the world
    ///
    /// With
    pub fn display(&self) {
        println!("\n\n-----------------------------------------------\n");
        self.stats();
        let mut grid = vec![vec![' '; WORLD_WIDTH as usize]; WORLD_HEIGHT as usize];
        for colony in &self.colonies {
            for (ant_type, ants) in &colony.ants {
                for ant in ants {
                    grid[ant.position.y_position as usize][ant.position.x_position as usize] =
                        match ant_type {
                            AntType::Scout => 'S',
                            AntType::Worker => 'W',
                        }
                }
            }
            grid[colony.position.y_position as usize][colony.position.x_position as usize] = 'C';
        }
        for coords in &self.food_lookup {
            grid[coords.x_position as usize][coords.y_position as usize] = 'F';
        }
        for line in grid {
            println!("{}", line.iter().collect::<String>());
        }
    }
    pub fn stats(&self) {
        println!("\n\n-----------------------------------------------\n");
        println!("    Number of Colonies: {}", self.colonies.len());
        for (index, colony) in self.colonies.iter().enumerate() {
            println!("        Colony: {}", index);
            for (ant_type, ants) in &colony.ants {
                println!("        Type: {} Number {}", ant_type, ants.len());
            }
        }
    }
}

/// This is a representation of a singular marker laid by ants
/// Should be updated every tick, and the strength reduces by the depreciation rate
#[derive(Copy, Clone)]
pub struct Pheromone {
    /// The current strength of the pheromone. Should be less than equal to the MAXIMUM_PHEROMONE_STRENGTH
    pub(crate) strength: u8,
    /// How much to reduce the strength by, per time step. Should be less than or equal to the strength
    depreciation_rate: u8,
}

impl Default for Pheromone {
    fn default() -> Self {
        Pheromone {
            strength: MAXIMUM_PHEROMONE_STRENGTH,
            depreciation_rate: 2,
        }
    }
}

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
    /// # let strength:u8=MAXIMUM_PHEROMONE_STRENGTH+1;
    /// # assert!(ant_lib::world::Pheromone::new(strength,1).is_none())
    /// ```
    pub fn new(strength: u8, depreciation_rate: u8) -> Option<Pheromone> {
        if MAXIMUM_PHEROMONE_STRENGTH < strength || strength < depreciation_rate {
            return None;
        }
        Some(Pheromone {
            strength,
            depreciation_rate,
        })
    }
    /// Used to increment the strength of a pheromone
    ///
    /// i.e. An ant walks over an existing pheromone and increases the strength by the given amount
    pub fn refresh(&mut self, strength: u8) {
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
}

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
    spawn_rate: u8,
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
                total_required_ants += required as u8;
            }
        }
        if DEBUG_MODE {
            println!("Total ants to spawn: {}", total_required_ants);
            println!("Spawn rate: {}", self.spawn_rate);
        }
        // Allocates and spawns the number of ants that can be spawned this turn, between the number of ants that are required per type
        for (ant_type, mut amount) in ants_spawn {
            amount = (amount as f64 * (self.spawn_rate as f64 / total_required_ants as f64)) as u16;
            if DEBUG_MODE {
                println!(
                    "Need to spawn: {} for type: {} at Position {}",
                    amount, ant_type, self.position
                );
            }
            let ant_container = self
                .ants
                .get_mut(&ant_type)
                .unwrap_or_else(|| panic!("Failed to get ant type {}", ant_type));
            for _ in 0..amount {
                ant_container.push(Ant::new(ant_type, self.position));
            }
        }
    }

    /// Spawns the maximum amount of ants it can this step
    ///
    /// And updates the position of all the colonies ants
    pub fn update(
        &mut self,
        food_map: &mut [[Option<Food>; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
        pheromones_lookup: &mut Vec<Coordinates>,
        pheromones_map: &mut [[Option<Pheromone>; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
    ) {
        self.spawn_ants();
        for (_, ants) in self.ants.iter_mut() {
            for ant in ants {
                ant.update(food_map, pheromones_lookup, pheromones_map);
            }
        }
    }
}
#[derive(Copy, Clone)]
pub struct Food {
    resources_remaining: u8,
}
impl Default for Food {
    fn default() -> Self {
        Food {
            resources_remaining: DEFAULT_RESOURCE_SIZE,
        }
    }
}
impl Food {
    fn consume(&mut self) -> Option<()> {
        if let Some(resources) = self.resources_remaining.checked_sub(1) {
            self.resources_remaining = resources;
            Some(())
        } else {
            None
        }
    }
}

pub struct Ant {
    ant_type: AntType,
    pub position: Coordinates,
}

const MOVE_POSSIBILITIES: [(i16, i16); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
impl Ant {
    /// Creates a new ant, with the given type and position, if it is inside the world boundary
    ///
    /// # Examples
    /// ```
    /// # use ant_lib::ant_settings::WORLD_WIDTH;
    /// # use ant_lib::world::{Ant, AntType, Coordinates};
    ///
    /// let position = Coordinates::new(0,5).unwrap();
    /// let ant_type=AntType::Scout;
    ///
    /// let ant = Ant::new(ant_type,position);
    /// ```
    pub fn new(ant_type: AntType, position: Coordinates) -> Ant {
        Ant { ant_type, position }
    }
    /// This will:
    /// * Move the ant
    /// * Update any relevant pheromones
    /// * Consume any available food
    pub fn update(
        &mut self,
        food_map: &mut [[Option<Food>; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
        pheromones_lookup: &mut Vec<Coordinates>,
        pheromones_map: &mut [[Option<Pheromone>; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
    ) {
        self.move_ant(pheromones_map);

        // Update the strength of pheromones
        if let Some(mut pheromone) =
            &pheromones_map[self.position.x_position as usize][self.position.y_position as usize]
        {
            pheromone.refresh(pheromone.strength);
        } else {
            pheromones_map[self.position.x_position as usize][self.position.y_position as usize] =
                Some(Pheromone::default());
            pheromones_lookup.push(self.position);
            if DEBUG_MODE {
                println!("New Pheromone at: {}", self.position)
            }
        }

        // Consume food if it is available
        if let Some(mut food) =
            &food_map[self.position.x_position as usize][self.position.y_position as usize]
        {
            if food.consume().is_none() {
                food_map[self.position.x_position as usize][self.position.y_position as usize] =
                    None;
            }
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
        pheromones_map: &[[Option<Pheromone>; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
    ) {
        // TODO Use pheromones to influence ant direction
        println!("Old ant position: {}", self.position);
        let random_chance: f64 = rand::random();
        if random_chance > self.ant_type.get_randomness_chance() {
            self.move_pheromones(pheromones_map);
        } else {
            self.move_random();
        }
        println!("Moved ant to position: {}", self.position)
    }
    /// Moves the ant in one of the possible directions given by: MOVE_POSSIBILITIES
    fn move_random(&mut self) {
        let direction: f64 = rand::random();
        let direction = (direction * 4.0) as usize;
        self.position = self.position.modify(
            MOVE_POSSIBILITIES[direction].0,
            MOVE_POSSIBILITIES[direction].1,
        );
    }
    /// Moves the ant in the direction of the strongest pheromone (of the possible directions given by: MOVE_POSSIBILITIES)
    ///
    /// If there are no nearby pheromones then, moves in a random direction
    fn move_pheromones(
        &mut self,
        pheromones_map: &[[Option<Pheromone>; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
    ) {
        let mut strongest_pheromone = 0;
        let mut position = Coordinates::default();
        for move_possibility in &MOVE_POSSIBILITIES {
            let new_position = self.position.modify(move_possibility.0, move_possibility.1);
            if let Some(pheromone) =
                pheromones_map[new_position.x_position as usize][new_position.y_position as usize]
            {
                if pheromone.strength > strongest_pheromone {
                    strongest_pheromone = pheromone.strength;
                    position = new_position;
                }
            }
        }
        if strongest_pheromone == 0 {
            let direction: f64 = rand::random();
            let direction = (direction * 4.0) as usize;
            position = self.position.modify(
                MOVE_POSSIBILITIES[direction].0,
                MOVE_POSSIBILITIES[direction].1,
            );
        }
        self.position = position;
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum AntType {
    Scout,
    Worker,
}
impl AntType {
    fn get_maximum_number_of_ants(&self) -> u8 {
        match self {
            AntType::Scout => DEFAULT_COLONY_SCOUT_SIZE,
            AntType::Worker => DEFAULT_COLONY_WORKER_SIZE,
        }
    }
    /// Returns the probability of the given ant type moving in a random direction
    fn get_randomness_chance(&self) -> f64 {
        match self {
            AntType::Scout => 0.75,
            AntType::Worker => 0.1,
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

#[derive(Copy, Clone)]
pub struct Coordinates {
    pub x_position: u16,
    pub y_position: u16,
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
    /// Will return the new ant coordinates after adjusting by the given amount, whilst staying in the world boundaries:
    /// (0..WORLD_WIDTH),(0..WORLD_HEIGHT)
    ///
    /// # Returns
    /// The new (x_position, y_position)
    pub fn modify(&self, x_amount: i16, y_amount: i16) -> Coordinates {
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
}
