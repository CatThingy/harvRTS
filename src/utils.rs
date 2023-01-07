use bevy::prelude::*;

use crate::MainCamera;

#[derive(Default, Deref, Resource)]
pub struct MousePosition(pub Vec3);

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
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosition>()
            .add_system(Self::update_mouse_position);
    }
}
