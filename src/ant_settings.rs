// TODO Load from JSON file
/// The width of the world in "tiles"
pub const WORLD_WIDTH: u16 = 64;
/// The height of the world in "tiles"
pub const WORLD_HEIGHT: u16 = 64;

/// The maximum strength that can be assigned to a pheromone
pub const MAXIMUM_PHEROMONE_STRENGTH: u16 = 1000;

/// The default amount to increase a pheromone, when walked over by another ant
pub const DEFAULT_PHEROMONE_REINFORCEMENT_AMOUNT: u16 = 10;

/// The default rate for exploration pheromones to dissipate
pub const DEFAULT_EXPLORATION_PHEROMONE_DEPRECIATION_RATE: u16 = 5;
/// The default rate for food pheromones to dissipate
pub const DEFAULT_FOOD_PHEROMONE_DEPRECIATION_RATE: u16 = 10;

/// The amount of scouts a default colony should aim to spawn
pub const DEFAULT_COLONY_SCOUT_SIZE: u16 = 25;
/// The amount of workers a default colony should aim to spawn
pub const DEFAULT_COLONY_WORKER_SIZE: u16 = 10;
/// The maximum amount of ants that can be spawned, per time step
pub const DEFAULT_COLONY_SPAWN_RATE: u16 = 2;

pub const DEFAULT_TERRITORY_SIZE: u16 = 0;

/// The default size of food
pub const DEFAULT_RESOURCE_SIZE: u8 = 20;
/// The amount of individual resource locations to spawn
pub const DEFAULT_RESOURCE_COUNT: u8 = 5;

/// The probability of scouts returning to the nest following pheromones
pub const SCOUT_RETURN_PHEROMONE_CHANCE: f64 = 0.9;
/// The probability of workers  following resource pheromones
pub const WORKER_PHEROMONE_CHANCE: f64 = 0.9;
/// The probability of an ant going backwards
pub const ANT_BACKWARDS_CHANCE: f64 = 0.1;

pub const PHEROMONE_TYPES_COUNT: usize = 2;

/// The amount of steps a scout will take, before returning to the nest
pub const DEFAULT_MAX_ANT_STEPS: u16 = 1000;

pub const DEBUG_MODE: bool = false;
