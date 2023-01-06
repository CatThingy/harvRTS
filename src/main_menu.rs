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
            root.spawn(ButtonBundle {
                image: UiImage(assets.load("play.png")),
                style: Style {
                    size: Size {
                        width: Val::Px(128.0),
                        height: Val::Px(64.0),
                    },
                    ..default()
                },
                ..default()
            })
            .insert(BeginButton);
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
