use super::inputs::MainCamera;
use super::ui::{GameExitEvent, SimulationState};
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;
use bevy::window::PrimaryWindow;

const SPRITE_SIZE: f32 = 32.0;
const GRID_SIZE: i32 = 100;

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
                FixedUpdate,
                (set_cursor_world_position, cell_interaction)
                    .chain()
                    .run_if(on_timer(Duration::from_secs_f32(0.016))),
            )
            .add_systems(
                FixedUpdate,
                simulation_step.run_if(on_timer(Duration::from_secs_f32(0.2))),
            )
            .add_systems(Update, (set_simulation, unset_simulation))
            .add_systems(Update, exit_game);
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

fn set_cursor_world_position(
    window_query: Query<&Window, With<PrimaryWindow>>,
    main_camera_query: Query<(&Transform, &OrthographicProjection), With<MainCamera>>,
    mouse_button: Res<Input<MouseButton>>,
    mut mouse_world_pos_draw: ResMut<MouseWorldPositionDraw>,
    mut mouse_world_pos_erase: ResMut<MouseWorldPositionErase>,
    is_runing: Res<IsSimulationRunning>,
) {
    let window = window_query.get_single().unwrap();
    if !is_runing.0 {
        if let Some(pos) = window.cursor_position() {
            let (transform, proj) = main_camera_query.get_single().unwrap();
            let pos_world = get_mouse_world(pos, transform, window, proj);

            if mouse_button.pressed(MouseButton::Left) {
                *mouse_world_pos_draw = MouseWorldPositionDraw(Some((pos_world.x, pos_world.y)));
            }
            if mouse_button.pressed(MouseButton::Right) {
                *mouse_world_pos_erase = MouseWorldPositionErase(Some((pos_world.x, pos_world.y)));
            }
        }
    }
}

fn get_mouse_world(
    pos: Vec2,
    main_transform: &Transform,
    window: &Window,
    proj: &OrthographicProjection,
) -> Vec3 {
    let center = main_transform.translation.truncate();
    let half_width = (window.width() / 2.0) * proj.scale;
    let half_height = (window.height() / 2.0) * proj.scale;
    let left = center.x - half_width;
    let bottom = center.y + half_height;

    Vec3::new(left + pos.x * proj.scale, bottom - pos.y * proj.scale, 0.0)
}

fn cell_interaction(
    mut cell_query: Query<(&mut Cell, &mut Handle<Image>, &Transform)>,
    mut mouse_world_pos_draw: ResMut<MouseWorldPositionDraw>,
    mut mouse_world_pos_erase: ResMut<MouseWorldPositionErase>,
    sprite_images: Res<SpriteImage>,
    is_runing: Res<IsSimulationRunning>,
) {
    let mouse_draw = mouse_world_pos_draw.0.take();
    let mouse_erase = mouse_world_pos_erase.0.take();

    if !is_runing.0 && (mouse_draw.is_some() || mouse_erase.is_some()) {
        for (mut cell, mut sprite, transform) in cell_query.iter_mut() {
            if let Some(mouse_world_pos) = mouse_draw {
                if is_in_cell_bounds(
                    mouse_world_pos,
                    (transform.translation.x, transform.translation.y),
                    (SPRITE_SIZE / 2.0, SPRITE_SIZE / 2.0),
                ) {
                    cell.state = CellState::Alive;
                    *sprite = sprite_images.alive_cell.clone();
                }
            }

            if let Some(mouse_world_pos) = mouse_erase {
                if is_in_cell_bounds(
                    mouse_world_pos,
                    (transform.translation.x, transform.translation.y),
                    (SPRITE_SIZE / 2.0, SPRITE_SIZE / 2.0),
                ) {
                    cell.state = CellState::Empty;
                    *sprite = sprite_images.empty_cell.clone();
                }
            }
        }
    }
}

fn is_in_cell_bounds(xy: (f32, f32), center: (f32, f32), dims: (f32, f32)) -> bool {
    xy.0 >= center.0 - dims.0
        && xy.0 <= center.0 + dims.0
        && xy.1 >= center.1 - dims.1
        && xy.1 <= center.1 + dims.1
}

fn exit_game(
    mut exit_writer: EventWriter<AppExit>,
    mut game_exit_event_reader: EventReader<GameExitEvent>,
) {
    for _ in game_exit_event_reader.iter() {
        exit_writer.send(AppExit);
    }
}

fn simulation_step(
    mut cell_query: Query<(&mut Cell, &mut Handle<Image>)>,
    is_runing: Res<IsSimulationRunning>,
    sprite_images: Res<SpriteImage>,
) {
    if is_runing.0 {
        let mut life_grid: Vec<bool> = Vec::new();
        for (cell, _) in cell_query.iter_mut() {
            life_grid.push(match cell.state {
                CellState::Alive => true,
                CellState::Dead | CellState::Empty => false,
            })
        }

        for (id, (mut cell, mut sprite)) in cell_query.iter_mut().enumerate() {
            let mut neighbour_count = 0;
            let x = id as i32 % GRID_SIZE;
            let y = id as i32 / GRID_SIZE;

            for xi in (x - 1)..=(x + 1) {
                for yi in (y - 1)..=(y + 1) {
                    if (xi != x && yi != y)
                        && xi >= 0
                        && xi < GRID_SIZE
                        && yi >= 0
                        && yi < GRID_SIZE
                    {
                        let neighbour_id = xi + yi * GRID_SIZE;
                        if life_grid[neighbour_id as usize] {
                            neighbour_count += 1;
                        }
                    }
                }
            }

            if neighbour_count < 2 || neighbour_count > 3 {
                match cell.state {
                    CellState::Alive => {
                        cell.state = CellState::Dead;
                        *sprite = sprite_images.dead_cell.clone();
                    }
                    CellState::Dead | CellState::Empty => {}
                }
            }

            if neighbour_count == 3 {
                cell.state = CellState::Alive;
                *sprite = sprite_images.alive_cell.clone();
            }
        }
    }
}

fn set_simulation(
    mut start_sim: ResMut<IsSimulationRunning>,
    sim_state: Res<State<SimulationState>>,
) {
    if *sim_state == SimulationState::Start {
        start_sim.0 = true;
    }
}

fn unset_simulation(
    mut start_sim: ResMut<IsSimulationRunning>,
    sim_state: Res<State<SimulationState>>,
) {
    if *sim_state == SimulationState::Stop {
        start_sim.0 = false;
    }
}
