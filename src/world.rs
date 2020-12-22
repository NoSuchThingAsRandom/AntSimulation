use crate::ant_settings::{
    DEBUG_MODE, DEFAULT_COLONY_SCOUT_SIZE, DEFAULT_COLONY_SPAWN_RATE, DEFAULT_COLONY_WORKER_SIZE,
    DEFAULT_RESOURCE_SIZE, MAXIMUM_PHEROMONE_STRENGTH, WORLD_HEIGHT, WORLD_WIDTH,
};
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

/// A struct containing every entity in the world
///
/// All entities/objects are accessed through this
pub struct World {
    /// A container all active food objects
    pub food: BTreeMap<(u32, u32), Food>,
    /// A container for all active colonies
    colonies: Vec<Colony>,
    /// A container for all active pheromones (with their x/y positions)
    pub pheromones: BTreeMap<(u32, u32), Pheromone>,
}
impl Default for World {
    fn default() -> Self {
        World {
            food: BTreeMap::new(),
            colonies: vec![],
            pheromones: BTreeMap::new(),
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
    pub fn new(food: BTreeMap<(u32, u32), Food>, colonies: Vec<Colony>) -> World {
        World {
            food,
            colonies,
            pheromones: BTreeMap::new(),
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
            colony.update(&mut self.food, &mut self.pheromones);
        }
        self.display();
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
                    grid[ant.y_position as usize][ant.x_position as usize] = match ant_type {
                        AntType::Scout => 'S',
                        AntType::Worker => 'W',
                    }
                }
            }
            grid[colony.y_position as usize][colony.x_position as usize] = 'C';
        }
        for (position, _) in &self.food {
            grid[position.1 as usize][position.0 as usize] = 'F';
        }
        for line in grid {
            for tile in line {
                println!("{}", line.iter().collect::<String>());
            }
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
pub struct Pheromone {
    /// The current strength of the pheromone. Should be less than equal to the MAXIMUM_PHEROMONE_STRENGTH
    strength: u8,
    /// How much to reduce the strength by, per time step. Should be less than or equal to the strength
    depreciation_rate: u8,
}

impl Default for Pheromone {
    fn default() -> Self {
        Pheromone {
            strength: 100,
            depreciation_rate: 1,
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
    /// The x coordinate of the colony position
    x_position: u32,
    /// The y coordinate of the colony position
    y_position: u32,
    /// Stores all ants, by their type
    // TODO Switch to a faster map
    ants: HashMap<AntType, Vec<Ant>>,
    /// The maximum number of ants that can be spawned per time step
    spawn_rate: u8,
}

impl Default for Colony {
    fn default() -> Self {
        let mut ants = HashMap::new();
        ants.insert(AntType::Scout, Vec::new());
        ants.insert(AntType::Worker, Vec::new());
        Colony {
            x_position: WORLD_WIDTH / 2,
            y_position: WORLD_HEIGHT / 2,
            ants: ants,
            spawn_rate: DEFAULT_COLONY_SPAWN_RATE,
        }
    }
}
impl Colony {
    /// Builds a new colony at the given position
    pub fn new(x_position: u32, y_position: u32) -> Colony {
        Colony {
            x_position,
            y_position,
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
            let max_ants = ant_type.get_maximum_number_of_ants() as usize;
            let required = max_ants - ants.len();
            if DEBUG_MODE {
                println!(
                    "Type: {} has maximum of {} and required: {}",
                    ant_type, max_ants, required
                );
            }
            if required > 0 {
                ants_spawn.push((ant_type.clone(), required));
                total_required_ants += required as u8;
            }
        }
        if DEBUG_MODE {
            println!("Total ants to spawn: {}", total_required_ants);
            println!("Spawn rate: {}", self.spawn_rate);
        }
        // Allocates and spawns the number of ants that can be spawned this turn, between the number of ants that are required per type
        for (ant_type, mut amount) in ants_spawn {
            amount =
                (amount as f64 * (self.spawn_rate as f64 / total_required_ants as f64)) as usize;
            if DEBUG_MODE {
                println!("Need to spawn: {} for type: {}", amount, ant_type);
            }
            let ant_container = self
                .ants
                .get_mut(&ant_type)
                .unwrap_or_else(|| panic!("Failed to get ant type {}", ant_type));
            for _ in 0..amount {
                if let Some(ant) = Ant::new(ant_type, self.x_position, self.y_position) {
                    ant_container.push(ant);
                } else {
                    panic!(
                        "Cannot spawn ant at {},{} ",
                        self.x_position, self.y_position
                    )
                }
            }
        }
    }

    /// Spawns the maximum amount of ants it can this step
    ///
    /// And updates the position of all the colonies ants
    pub fn update(
        &mut self,
        food_map: &mut BTreeMap<(u32, u32), Food>,
        pheromones_map: &mut BTreeMap<(u32, u32), Pheromone>,
    ) {
        self.spawn_ants();
        for (ant_type, ants) in self.ants.iter_mut() {
            for ant in ants {
                ant.update(food_map, pheromones_map);
            }
        }
    }
}
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
    x_position: u32,
    y_position: u32,
}

impl Ant {
    /// Creates a new ant, with the given type and position, if it is inside the world boundary
    ///
    /// # Examples
    /// ```
    /// # use ant_lib::ant_settings::WORLD_WIDTH;
    /// # use ant_lib::world::{Ant, AntType};
    ///
    /// let x=5;
    /// let y=5;
    /// let ant_type=AntType::Scout;
    ///
    /// let ant=Ant::new(ant_type,x,y);
    /// # assert!(ant.is_some());
    /// ```
    ///```
    /// # // x value outside the borders of the world, should fail
    /// # use ant_lib::world::{AntType, Ant};
    /// # use ant_lib::ant_settings::WORLD_WIDTH;
    ///
    /// # let x=WORLD_WIDTH+1;
    /// # let y=5;
    /// # let ant_type=AntType::Scout;
    ///
    /// # assert!(Ant::new(ant_type,x,y).is_none());
    ///```
    /// ```
    /// # //Y value outside the borders of the world, should fail
    /// # use ant_lib::ant_settings::WORLD_HEIGHT;
    /// # use ant_lib::world::{Ant, AntType};
    ///
    /// # let x=5;
    /// # let y=WORLD_HEIGHT+1;
    /// # let ant_type=AntType::Scout;
    ///
    /// # assert!(Ant::new(ant_type,x,y).is_none());
    /// ```
    pub fn new(ant_type: AntType, x_position: u32, y_position: u32) -> Option<Ant> {
        if x_position > WORLD_WIDTH || y_position > WORLD_HEIGHT {
            return None;
        }
        Some(Ant {
            ant_type,
            x_position,
            y_position,
        })
    }
    /// This will:
    /// * Move the ant
    /// * Update any relevant pheromones
    /// * Consume any available food
    pub fn update(
        &mut self,
        food_map: &mut BTreeMap<(u32, u32), Food>,
        pheromones_map: &mut BTreeMap<(u32, u32), Pheromone>,
    ) {
        // TODO Use pheromones to influence ant direction
        let direction: f64 = rand::random();
        let direction = (direction * 4.0) as u8;
        match direction {
            0 => self.move_ant(-1, 0),
            1 => self.move_ant(1, 0),
            2 => self.move_ant(0, 1),
            3 => self.move_ant(0, -1),
            _ => {}
        }
        // Update the strength of pheromones
        if let Some(pheromone) = pheromones_map.get_mut(&(self.x_position, self.y_position)) {
            pheromone.refresh(pheromone.strength);
        } else {
            pheromones_map.insert((self.x_position, self.y_position), Pheromone::default());
        }

        // Consume food if it is available
        if let Some(food) = food_map.get_mut(&(self.x_position, self.y_position)) {
            if food.consume().is_none() {
                food_map.remove(&(self.x_position, self.y_position));
            }
        }
    }
    /// Will safely move the ant by the given amount, whilst staying in the world boundaries
    /// (0..WORLD_WIDTH),(0..WORLD_HEIGHT)
    pub fn move_ant(&mut self, x_distance: i32, y_distance: i32) {
        // TODO When "if let Some(x) && x condition" is stable, change this
        if x_distance.is_positive() {
            let new_position = self.x_position.checked_add(x_distance as u32);
            if let Some(position) = new_position {
                if position < WORLD_WIDTH {
                    self.x_position = position;
                } else {
                    self.x_position = WORLD_WIDTH - 1;
                }
            } else {
                self.x_position = WORLD_WIDTH - 1;
            }
        } else {
            let new_position = (self.x_position as i32).checked_sub(x_distance);

            if let Some(position) = new_position {
                if position > 0 {
                    self.x_position = position as u32;
                } else {
                    self.x_position = 0;
                }
            } else {
                self.x_position = 0;
            }
        }
        if y_distance.is_positive() {
            let new_position = self.y_position.checked_add(y_distance as u32);
            if let Some(position) = new_position {
                if position < WORLD_WIDTH {
                    self.y_position = position;
                } else {
                    self.y_position = WORLD_WIDTH - 1;
                }
            } else {
                self.y_position = WORLD_WIDTH - 1;
            }
        } else {
            let new_position = (self.y_position as i32).checked_sub(y_distance);

            if let Some(position) = new_position {
                if position > 0 {
                    self.y_position = position as u32;
                } else {
                    self.y_position = 0;
                }
            } else {
                self.y_position = 0;
            }
        }
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
}
impl Display for AntType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AntType::Scout => write!(f, "Scout"),
            AntType::Worker => write!(f, "Worker"),
        }
    }
}
