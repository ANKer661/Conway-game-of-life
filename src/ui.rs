use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.4, 0.8, 0.8);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.1, 1.0, 1.0);

#[derive(Event)]
pub struct GameExitEvent;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Stop,
    Start,
}

#[derive(Component)]
pub struct ClassicButton(ButtonType);

pub enum ButtonType {
    Play,
    Stop,
    Exit,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameExitEvent>()
            .add_state::<SimulationState>()
            .add_systems(Startup, setup)
            .add_systems(Update, button_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI root
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        // Bottom button BG border
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(100.0),
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                    ..default()
                })
                // Bottom button fill
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::FlexEnd,
                                ..default()
                            },
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
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(50.0),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
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
                TextStyle {
                    font: asset_server.load("fonts/Symtext.ttf"),
                    font_size: 32.0,
                    color: Color::WHITE.into(),
                },
            )],
            ..default()
        },
        ..default()
    }
}

fn button_system(
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
