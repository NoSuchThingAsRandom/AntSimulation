use sim::ant::AntType;
use sim::pheromone::{Pheromone, PheromoneType};
use ggez::graphics::Color;
pub use sim::ant_settings::{
    MAXIMUM_PHEROMONE_STRENGTH
};

/// Returns the colour to render the given Ant Type as
pub fn get_ant_color(ant: &AntType) -> Color {
    match ant {
        AntType::Scout => Color::from_rgb(0, 0, 255),
        AntType::Worker => Color::from_rgb(50, 190, 190),
    }
}

/// Returns the color that the Pheromone should be rendered as
///
/// The lightness depends on the strength of the Pheromone
pub fn get_pheromone_color(pheromone: &Pheromone) -> Color {
    let color = (200_f64 * ((pheromone.get_strength() as f64) / (MAXIMUM_PHEROMONE_STRENGTH as f64)))
        as u8
        + 55;
    match pheromone.pheromone_type {
        PheromoneType::Exploration => Color::from_rgb(color, 0, color),
        PheromoneType::Resource => Color::from_rgb(color, color, color),
    }
}
