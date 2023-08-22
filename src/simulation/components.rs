use bevy::prelude::Component;

#[derive(Component)]
pub struct Cell {
    pub state: CellState,
}

pub enum CellState {
    Alive,
    Dead,
    Empty,
}
