use bevy::prelude::*;

use crate::{health::Dead, MainCamera};

#[derive(Default, Deref, Resource)]
pub struct MousePosition(pub Vec3);

#[derive(Component)]
pub struct Bar {
    pub value: f32,
    pub max: f32,
    pub size: f32,
}

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
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosition>()
            .add_system(Self::update_bar)
            .add_system(Self::update_mouse_position)
            .add_system(Self::handle_dead);
    }
}
