use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const CAMERA_MOVE_SPEED: f32 = 15.0;
const CAMERA_ZOOM_SPEED: f32 = 0.75;

#[derive(Component)]
pub struct MainCamera {}

#[derive(Component)]
pub struct Movement {
    plain_speed: Vec3,
    zoom_speed: f32,
}

pub struct InputsPlugin;

impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(0.033))
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, (camera_move, camera_zoom));
    }
}

fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0),
            ..default()
        },
        MainCamera {},
        Movement {
            plain_speed: Vec3::new(0.0, 0.0, 0.0),
            zoom_speed: 0.0,
        },
    ));
}

fn camera_move(
    mut camera_query: Query<(&mut Transform, &mut Movement), With<MainCamera>>,
    keboard_inputs: Res<Input<KeyCode>>,
) {
    let mut move_direction = Vec3::new(0.0, 0.0, 0.0);
    if keboard_inputs.pressed(KeyCode::W) {
        move_direction.y += 1.0;
    }
    if keboard_inputs.pressed(KeyCode::A) {
        move_direction.x -= 1.0;
    }
    if keboard_inputs.pressed(KeyCode::S) {
        move_direction.y -= 1.0;
    }
    if keboard_inputs.pressed(KeyCode::D) {
        move_direction.x += 1.0;
    }
    let move_direction = move_direction.normalize_or_zero();

    let (mut transform, mut movement) = camera_query
        .iter_mut()
        .next()
        .expect("No transform on main camera.");

    movement.plain_speed = (movement.plain_speed + move_direction).clamp(
        Vec3::new(-CAMERA_MOVE_SPEED, -CAMERA_MOVE_SPEED, -CAMERA_MOVE_SPEED),
        Vec3::new(CAMERA_MOVE_SPEED, CAMERA_MOVE_SPEED, CAMERA_MOVE_SPEED),
    );

    if keboard_inputs.pressed(KeyCode::Space) {
        movement.plain_speed = Vec3::new(0.0, 0.0, 0.0);
    }

    transform.translation += movement.plain_speed;
}

fn camera_zoom(
    mut camera_query: Query<(&mut Movement, &mut OrthographicProjection), With<MainCamera>>,
    keboard_inputs: Res<Input<KeyCode>>,
) {
    let mut zoom_direction = 0.0;
    if keboard_inputs.pressed(KeyCode::Q) {
        zoom_direction += 0.01;
    }
    if keboard_inputs.pressed(KeyCode::E) {
        zoom_direction -= 0.01;
    }

    let (mut movement, mut orth_proj) = camera_query.single_mut();

    movement.zoom_speed =
        (movement.zoom_speed + zoom_direction).clamp(-CAMERA_ZOOM_SPEED, CAMERA_ZOOM_SPEED);
    orth_proj.scale = (orth_proj.scale + movement.zoom_speed).clamp(1.0, 6.0);

    if (orth_proj.scale - 1.0).abs() < 0.0001
        || (orth_proj.scale - 6.0).abs() < 0.0001
        || keboard_inputs.pressed(KeyCode::Space)
    {
        movement.zoom_speed = 0.0;
    }
}
