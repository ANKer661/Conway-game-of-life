mod components;
mod styles;
pub mod systems;

use self::systems::{button_system, setup, GameExitEvent, SimulationState};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameExitEvent>()
            .add_state::<SimulationState>()
            .add_systems(Startup, setup)
            .add_systems(Update, button_system);
    }
}
