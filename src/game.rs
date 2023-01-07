use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct Rose;

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

                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.5)),
                ..default()
            },
            Rose,
        ));
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, Self::init);
    }
}
