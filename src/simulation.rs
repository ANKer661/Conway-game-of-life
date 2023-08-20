use super::ui::{GameExitEvent, SimulationState};
use bevy::prelude::*;

const SPRITE_SIZE: f32 = 32.0;
const GRID_SIZE: u32 = 100;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.156, 0.172, 0.203)))
            .insert_resource(MouseWorldPositionDraw(None))
            .insert_resource(MouseWorldPositionErase(None))
            .insert_resource(IsSimulationRunning(false))
            .add_systems(Startup, setup);
    }
}

#[derive(Default, Resource)]
struct MouseWorldPositionDraw(Option<(f32, f32)>);

#[derive(Default, Resource)]
struct MouseWorldPositionErase(Option<(f32, f32)>);

#[derive(Component)]
struct Cell {
    state: CellState,
}

enum CellState {
    Alive,
    Dead,
    Empty,
}

#[derive(Default, Resource)]
struct SpriteImage {
    empty_cell: Handle<Image>,
    alive_cell: Handle<Image>,
    dead_cell: Handle<Image>,
}

#[derive(Default, Resource)]
struct IsSimulationRunning(bool);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("sprites/empty_cell.png"),
                    transform: Transform {
                        translation: Vec3::new(
                            (x as f32) * SPRITE_SIZE,
                            (y as f32) * SPRITE_SIZE,
                            0.0,
                        ),
                        scale: Vec3::new(1.0, 1.0, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Cell {
                    state: CellState::Empty,
                },
            ));
        }
    }

    commands.insert_resource(SpriteImage {
        empty_cell: asset_server.load("sprites/empty_cell.png"),
        alive_cell: asset_server.load("sprites/alive_cell.png"),
        dead_cell: asset_server.load("sprites/dead_cell.png"),
    });
}
