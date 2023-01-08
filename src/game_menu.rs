use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{utils::Reset, GameState};

#[derive(Component)]
pub struct Root;

#[derive(Component)]
pub struct GameMenu;

#[derive(Component)]
pub struct DeathText;

#[derive(Component)]
pub struct GameTimer;

#[derive(Component)]
pub struct MainMenuButton;

pub struct Plugin;

impl Plugin {
    fn init(mut cmd: Commands, assets: Res<AssetServer>) {
        cmd.spawn((
            NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                    },
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            Root,
        ))
        .with_children(|root| {
            root.spawn((
                TextBundle::from_section(
                    "0:00",
                    TextStyle {
                        font: assets.load("fonts/ModeSeven.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                GameTimer,
            ));

            root.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::None,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        size: Size {
                            width: Val::Percent(75.0),
                            height: Val::Percent(75.0),
                        },
                        position: UiRect::all(Val::Percent(12.5)),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    background_color: Color::DARK_GRAY.into(),
                    ..default()
                },
                GameMenu,
            ))
            .with_children(|panel| {
                panel
                    .spawn(NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::SpaceEvenly,
                            size: Size {
                                width: Val::Percent(100.0),
                                height: Val::Auto,
                            },
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    })
                    .with_children(|panel| {
                        panel.spawn((
                            ImageBundle {
                                style: Style {
                                    // display: Display::None,
                                    justify_content: JustifyContent::SpaceEvenly,
                                    size: Size {
                                        width: Val::Px(512.0),
                                        height: Val::Px(128.0),
                                    },
                                    ..default()
                                },
                                image: UiImage(assets.load("death_text.png")),
                                ..default()
                            },
                            DeathText,
                        ));
                    });
                panel
                    .spawn(NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::SpaceEvenly,
                            size: Size {
                                width: Val::Percent(100.0),
                                height: Val::Auto,
                            },
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    })
                    .with_children(|panel| {
                        panel.spawn((
                            ButtonBundle {
                                image: UiImage(assets.load("menu_button.png")),
                                style: Style {
                                    size: Size {
                                        width: Val::Px(256.0),
                                        height: Val::Px(64.0),
                                    },
                                    ..default()
                                },
                                ..default()
                            },
                            MainMenuButton,
                        ));
                    });
            });
        });
    }

    fn handle_menu_click(
        mut cmd: Commands,
        mut event_writer: EventWriter<Reset>,
        q_button: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
        mouse: Res<Input<MouseButton>>,
    ) {
        if mouse.just_released(MouseButton::Left) {
            for button in &q_button {
                if button == &Interaction::Hovered {
                    event_writer.send(Reset);
                    cmd.insert_resource(NextState(GameState::MainMenu))
                }
            }
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, Self::init)
            .add_system(Self::handle_menu_click.run_in_state(GameState::InGame));
    }
}
