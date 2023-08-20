use bevy::prelude::*;

mod ui;
mod inputs;
mod simulation;

use ui::MainMenuPlugin;
use inputs::InputsPlugin;
use simulation::SimulationPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1024.0, 720.0).into(),
                title: "Conway's Game Of Life".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MainMenuPlugin)
        .add_plugins(InputsPlugin)
        .add_plugins(SimulationPlugin)
        .run();
}
