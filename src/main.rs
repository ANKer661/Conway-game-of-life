mod inputs;
mod simulation;
mod ui;

use self::inputs::InputsPlugin;
use self::simulation::SimulationPlugin;
use self::ui::MainMenuPlugin;
use bevy::prelude::*;

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
