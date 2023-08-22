mod components;
mod resources;
mod systems;

use self::resources::*;
use self::systems::*;
use crate::ui::systems::SimulationState;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .insert_resource(ClearColor(Color::rgb(0.156, 0.172, 0.203)))
            .insert_resource(MouseWorldPositionDraw(None))
            .insert_resource(MouseWorldPositionErase(None))
            .insert_resource(IsSimulationRunning(false))
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (set_cursor_world_position, cell_interaction)
                    .chain()
                    .run_if(on_timer(Duration::from_secs_f32(0.016))),
            )
            .add_systems(
                Update,
                simulation_step
                    .run_if(on_timer(Duration::from_secs_f32(0.1)))
                    .run_if(in_state(SimulationState::Start)),
            )
            .add_systems(Update, (set_simulation, unset_simulation))
            .add_systems(Update, exit_game);
    }
}
