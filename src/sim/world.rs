use crate::ant_settings::{
    DEFAULT_RESOURCE_COUNT, PHEROMONE_TYPES_COUNT, WORLD_HEIGHT, WORLD_WIDTH,
};
use crate::sim::ant::AntType;
use crate::sim::colony::Colony;
use crate::sim::pheromone::{Pheromone, PheromoneType};
use crate::sim::resource::Resource;
use crate::sim::Coordinates;

/// A struct containing every entity in the world
///
/// All entities/objects are accessed through this
pub struct World {
    /// A container all active food objects
    pub resources: [[Option<Resource>; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
    pub resource_lookup: Vec<Coordinates>,
    /// A container for all active colonies
    pub colonies: Vec<Colony>,
    /// A container for all active pheromones (with their x/y positions)
    pub pheromones:
        [[[Option<Pheromone>; PHEROMONE_TYPES_COUNT]; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
    pub pheromone_lookup: Vec<(Coordinates, PheromoneType)>,
}
impl Default for World {
    fn default() -> Self {
        let mut world = World {
            resources: [[None; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize],
            resource_lookup: Vec::new(),
            colonies: vec![],
            pheromones: [[[None; PHEROMONE_TYPES_COUNT]; WORLD_HEIGHT as usize];
                WORLD_WIDTH as usize],
            pheromone_lookup: Vec::new(),
        };
        world.new_colony();
        for _ in 0..DEFAULT_RESOURCE_COUNT {
            world.new_resource();
        }
        world
    }
}

impl World {
    /// Creates a new World instance with the supplied arguments
    ///
    /// # Arguments
    /// * `food*` A vector with all food instances that should exist on creation
    /// * `colonies*` A vector with all colonies instances that should exist on creation
    ///
    pub fn new(food: Vec<(Coordinates, Resource)>, colonies: Vec<Colony>) -> World {
        let mut food_container = [[None; WORLD_HEIGHT as usize]; WORLD_WIDTH as usize];
        let mut food_lookup = Vec::new();
        for (coords, food_entry) in food {
            food_container[coords.x_position as usize][coords.y_position as usize] =
                Some(food_entry);
            food_lookup.push(coords);
        }
        World {
            resources: food_container,
            resource_lookup: food_lookup,
            colonies,
            pheromones: [[[None; PHEROMONE_TYPES_COUNT]; WORLD_HEIGHT as usize];
                WORLD_WIDTH as usize],
            pheromone_lookup: Vec::new(),
        }
    }
    /// Creates a new default colony, and adds it to the world
    pub fn new_colony(&mut self) {
        self.colonies.push(Colony::default());
    }
    /// Spawns a new resource at a random location
    ///
    /// That is not occupied by another resource
    pub fn new_resource(&mut self) {
        let mut coords = Coordinates::new_random();
        while self.resources[coords.get_x_position_usize()][coords.get_y_position_usize()].is_some()
        {
            coords = Coordinates::new_random();
        }
        self.resources[coords.get_x_position_usize()][coords.get_y_position_usize()] =
            Some(Resource::default());
        self.resource_lookup.push(coords);
    }

    /// The main updater method
    /// This will:
    /// * Spawn any new food/ants if required
    /// * Update the position of ants
    /// * Update the strength of pheromones and remove them if necessary
    pub fn update(&mut self) {
        for colony in &mut self.colonies {
            colony.update(
                &mut self.resources,
                &mut self.pheromone_lookup,
                &mut self.pheromones,
            );
        }
        let mut new_lookup = self.pheromone_lookup.clone();
        new_lookup.retain(|(coords, pheromone_type)| {
            let mut retain = true;
            if let Some(pheromones) = &mut self.pheromones[coords.x_position as usize]
                [coords.y_position as usize][pheromone_type.as_pheromone_index()]
            {
                retain = pheromones.update();
            }
            if !retain {
                self.pheromones[coords.x_position as usize][coords.y_position as usize]
                    [pheromone_type.as_pheromone_index()] = None;
            }
            retain
        });
        self.pheromone_lookup = new_lookup;
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
        for coords in &self.resource_lookup {
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
