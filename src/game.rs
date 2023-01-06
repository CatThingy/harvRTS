use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{consts::PLOT_SIZE, GameState};

#[derive(Component)]
pub struct Rose;

#[derive(Component)]
pub struct Plot;

pub struct Plugin;

impl Plugin {
    fn init(mut cmd: Commands, assets: Res<AssetServer>) {
        cmd.spawn((
            SpriteBundle {
                texture: assets.load("rose.png"),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::BottomCenter,
                    ..default()
                },

                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                ..default()
            },
            Rose,
        ));

        for offset in [
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(-1.0, 0.0),
            Vec2::new(0.0, -1.0),
        ] {
            cmd.spawn((
                SpriteBundle {
                    texture: assets.load("plot.png"),
                    sprite: Sprite {
                        custom_size: Some(PLOT_SIZE),
                        ..default()
                    },

                    transform: Transform::from_translation(Vec3::new(
                        offset.x * PLOT_SIZE.x,
                        offset.y * PLOT_SIZE.y,
                        0.0,
                    )),
                    ..default()
                },
                Plot,
            ));
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, Self::init);
    }
}
