use super::components::{ButtonType, ClassicButton};
use super::styles::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct GameExitEvent;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Stop,
    Start,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI root
    commands
        .spawn(NodeBundle {
            style: UI_ROOT_STYLE,
            background_color: Color::NONE.into(),
            ..default()
        })
        // Bottom button BG border
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: BUTTON_BG_STYLE,
                    background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                    ..default()
                })
                // Bottom button fill
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: BUTTON_FILL_STYLE,
                            background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    build_classic_button(&asset_server),
                                    ClassicButton(ButtonType::Play),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(build_classic_text("PLAY", &asset_server));
                                });

                            parent
                                .spawn((
                                    build_classic_button(&asset_server),
                                    ClassicButton(ButtonType::Stop),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(build_classic_text("STOP", &asset_server));
                                });

                            parent
                                .spawn((
                                    build_classic_button(&asset_server),
                                    ClassicButton(ButtonType::Exit),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(build_classic_text("EXIT", &asset_server));
                                });
                        });
                });
        });
}

fn build_classic_button(asset_server: &Res<AssetServer>) -> ButtonBundle {
    ButtonBundle {
        style: CLASSIC_BUTTON_STYLE,
        background_color: NORMAL_BUTTON_COLOR.into(),
        image: UiImage {
            texture: asset_server.load("sprites/button.png"),
            ..default()
        },
        ..default()
    }
}

fn build_classic_text(text: &str, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![TextSection::new(
                text,
                get_classic_text_style(&asset_server),
            )],
            ..default()
        },
        ..default()
    }
}

pub fn button_system(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &ClassicButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_appstate: ResMut<NextState<SimulationState>>,
    mut exit_writer: EventWriter<GameExitEvent>,
) {
    for (interaction, mut background_color, classic_button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                match classic_button.0 {
                    ButtonType::Play => next_appstate.set(SimulationState::Start),
                    ButtonType::Stop => next_appstate.set(SimulationState::Stop),
                    ButtonType::Exit => exit_writer.send(GameExitEvent),
                }
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
