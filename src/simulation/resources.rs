use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct MouseWorldPositionDraw(pub Option<(f32, f32)>);

#[derive(Default, Resource)]
pub struct MouseWorldPositionErase(pub Option<(f32, f32)>);

#[derive(Default, Resource)]
pub struct SpriteImage {
    pub empty_cell: Handle<Image>,
    pub alive_cell: Handle<Image>,
    pub dead_cell: Handle<Image>,
}

#[derive(Default, Resource)]
pub struct IsSimulationRunning(pub bool);
