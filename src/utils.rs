use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    game::{Compost, Spawner},
    health::Dead,
    GameState, MainCamera,
};

#[derive(Default, Deref, Resource)]
pub struct MousePosition(pub Vec3);

#[derive(Component)]
pub struct Bar {
    pub value: f32,
    pub max: f32,
    pub size: f32,
}

pub struct Reset;

pub struct Plugin;
impl Plugin {
    fn update_mouse_position(
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        windows: Res<Windows>,
        mut mouse_pos: ResMut<MousePosition>,
    ) {
        let (camera, camera_transform) = q_camera.single();
        let Some(cursor_pos) = windows.primary().cursor_position() else { return };

        mouse_pos.0 = camera
            .viewport_to_world(camera_transform, cursor_pos)
            .unwrap()
            .origin;
        mouse_pos.0.z = 0.0;
    }

    fn update_bar(mut q_bar: Query<(&Bar, &mut Sprite)>) {
        for (bar, mut sprite) in &mut q_bar {
            sprite.custom_size = sprite
                .custom_size
                .map(|v| Vec2::new(bar.size * (bar.value / bar.max), v.y));
        }
    }

    fn handle_dead(mut cmd: Commands, q_dead: Query<Entity, With<Dead>>) {
        for dead in &q_dead {
            cmd.entity(dead).despawn_recursive();
        }
    }

    fn reset(
        mut cmd: Commands,
        mut time: ResMut<Time>,
        q_all: Query<Entity, (With<ComputedVisibility>, Without<Parent>)>,
    ) {
        cmd.insert_resource(Spawner {
            enemy: Timer::from_seconds(5.0, TimerMode::Repeating),
            total: Duration::default(),
        });
        cmd.insert_resource(Compost(100));

        time.set_relative_speed(1.0);

        for entity in &q_all {
            cmd.entity(entity).despawn_recursive();
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosition>()
            .init_resource::<Events<Reset>>()
            .add_exit_system(GameState::InGame, Self::reset)
            .add_system(Self::update_bar.run_in_state(GameState::InGame))
            .add_system(Self::update_mouse_position.run_in_state(GameState::InGame))
            .add_system(Self::handle_dead.run_in_state(GameState::InGame));
    }
}
