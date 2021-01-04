A simplistic attempt, at modelling ant movements and exploration.

Uses the ggez library for rendering

To run the sim, download the repositry, cargo and rust stable 1.47.
Then inside the Render directory, execute the command, "cargo run --release"

**Simulator Logic**

* Every time step calls an update to the World instance.
* Which then calls update on every colony.
    * The colony will attempt to spawn as many ants as it can (given by the spawn_rate),

    * The type of ant spawned is distributed, based on how many are missing from the required target, set in
      ant_settings.rs
    * `(DEFAULT_COLONY_"ANT_TYPE"_SIZE - active_"ANT_TYPE"_size)`

    * Then it updates the movements of each ant:


* And it also reduces the strength of every pheromone by their individual depreciation rate, and if the strength reaches
  zero, deletes the pheromone.

**Colour Code**

* Pink - Exploration Pheromone, where lighter is stronger
* White - Resource Pheromone, where lighter is stronger
* Green - Resource, where lighter greens, are less depleted resources
* Red - A colony
* Dark Blue - Scout
* Light Blue - Worker

**Code Structure**

All rendering logic takes place in the render crate file

The sim crate is responsible for the actual updating of state

The ant_settings.rs file in the sim crate is currently where all global defaults are defined. A future iteration would move this to a
JSON or equivalent.

**Glossary**

* Scout - An ant specialised in finding new resources
* Worker - An ant specialised for traversing between resources and the colony
* Resource - A cell on the map that the colony targets and "extracts" from
* Colony - The cell where ants are spawned, and bring resources to

**Useful Links**:

* https://softologyblog.wordpress.com/2020/03/21/ant-colony-simulations/
* https://itp.uni-frankfurt.de/~gros/StudentProjects/Applets_2014_AntsSimulation/ants.htm
* https://users.csc.calpoly.edu/~zwood/teaching/csc570/final13/smarano/


**Notes**

Currently on works on rustc <=1.47.0, due to a bug with 1.48 and winit 0.19

If necessary, to use newer versions of rustc then, ggez development branch can be used

See: https://github.com/ggez/ggez/issues/843

Also world size between 32 and 75 take a significantly long time to compile (on release mode), due to some optimisation issues with the compiler
See: https://github.com/rust-lang/rust/issues/80622
