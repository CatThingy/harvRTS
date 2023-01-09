use bevy::prelude::*;

use crate::GameState;
use iyes_loopless::prelude::*;

#[derive(Component)]
pub struct Root;

#[derive(Component)]
pub struct BeginButton;

pub struct Plugin;

impl Plugin {
    fn init(mut cmd: Commands, assets: Res<AssetServer>) {
        cmd.spawn(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(Root)
        .with_children(|root| {
            root.spawn(ImageBundle {
                image: UiImage(assets.load("title.png")),
                style: Style {
                    size: Size {
                        width: Val::Px(768.0),
                        height: Val::Px(320.0),
                    },
                    ..default()
                },
                ..default()
            });
            root.spawn((
                ButtonBundle {
                    image: UiImage(assets.load("play.png")),
                    style: Style {
                        size: Size {
                            width: Val::Px(128.0),
                            height: Val::Px(64.0),
                        },
                        ..default()
                    },
                    ..default()
                },
                BeginButton,
            ));
            root.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Click the plots to plant crops.\n".to_owned()
                                + "When fully grown, crops can be either harvested, turning into units, or composted.\n"
                                + "If not harvested, fully-grown crops will decay, providing half of their compost value.\n"
                                + "Click and drag to select units, right click to move them, and press A to command them to attack towards your cursor.",
                            style: TextStyle {
                                font: assets.load("fonts/ModeSeven.ttf"),
                                font_size: 20.0,
                                color: Color::BLACK,
                            },
                        }],
                        ..default()
                    },
                            style: Style {
                                max_size: Size::new(Val::Px(700.0), Val::Px(400.0)),
                                ..default()
                            },
                    ..default()
            });
        });
    }

    fn cleanup(mut cmd: Commands, q_root: Query<Entity, With<Root>>) {
        for entity in &q_root {
            cmd.entity(entity).despawn_recursive();
        }
    }

    fn handle_play_click(
        mut cmd: Commands,
        q_button: Query<&Interaction, (Changed<Interaction>, With<BeginButton>)>,
        mouse: Res<Input<MouseButton>>,
    ) {
        if mouse.just_released(MouseButton::Left) {
            for button in &q_button {
                if button == &Interaction::Hovered {
                    cmd.insert_resource(NextState(GameState::InGame))
                }
            }
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::MainMenu, Self::init)
            .add_exit_system(GameState::MainMenu, Self::cleanup)
            .add_system(Self::handle_play_click.run_in_state(GameState::MainMenu));
    }
}
