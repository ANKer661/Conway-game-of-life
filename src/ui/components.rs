use bevy::prelude::Component;

#[derive(Component)]
pub struct ClassicButton(pub ButtonType);

pub enum ButtonType {
    Play,
    Stop,
    Exit,
}
