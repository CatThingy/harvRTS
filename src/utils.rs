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

pub struct PlaySound(pub String);

#[derive(Resource)]
pub struct Preload(Vec<HandleUntyped>);

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
            aphid: Timer::from_seconds(10.0, TimerMode::Repeating),
            caterpillar: Timer::from_seconds(100.0, TimerMode::Repeating),
            total: Duration::default(),
        });
        cmd.insert_resource(Compost(100));

        time.set_relative_speed(1.0);

        for entity in &q_all {
            cmd.entity(entity).despawn_recursive();
        }
    }

    fn play_sound(audio: Res<Audio>, assets: Res<AssetServer>, mut events: EventReader<PlaySound>) {
        for PlaySound(sound) in events.iter() {
            audio.play(assets.load(sound));
        }
    }

    fn preload(mut cmd: Commands, assets: Res<AssetServer>) {
        let mut preloaded = vec![];
        preloaded.push(assets.load_untyped("aphid.png"));
        preloaded.push(assets.load_untyped("arrow.png"));
        preloaded.push(assets.load_untyped("cancel.png"));
        preloaded.push(assets.load_untyped("carrot_growing.png"));
        preloaded.push(assets.load_untyped("carrot_grown.png"));
        preloaded.push(assets.load_untyped("carrot_unit.png"));
        preloaded.push(assets.load_untyped("caterpillar.png"));
        preloaded.push(assets.load_untyped("clear.ogg"));
        preloaded.push(assets.load_untyped("clover_growing.png"));
        preloaded.push(assets.load_untyped("clover_grown.png"));
        preloaded.push(assets.load_untyped("clover_unit.png"));
        preloaded.push(assets.load_untyped("compost.ogg"));
        preloaded.push(assets.load_untyped("compost.png"));
        preloaded.push(assets.load_untyped("death_text.png"));
        preloaded.push(assets.load_untyped("empty.png"));
        preloaded.push(assets.load_untyped("harvest.png"));
        preloaded.push(assets.load_untyped("menu_button.png"));
        preloaded.push(assets.load_untyped("plant.ogg"));
        preloaded.push(assets.load_untyped("plant_carrot.png"));
        preloaded.push(assets.load_untyped("plant_clover.png"));
        preloaded.push(assets.load_untyped("plant_wheat.png"));
        preloaded.push(assets.load_untyped("play.png"));
        preloaded.push(assets.load_untyped("plot.png"));
        preloaded.push(assets.load_untyped("plot_circle.png"));
        preloaded.push(assets.load_untyped("rocks.png"));
        preloaded.push(assets.load_untyped("rose.png"));
        preloaded.push(assets.load_untyped("snip.ogg"));
        preloaded.push(assets.load_untyped("title.png"));
        preloaded.push(assets.load_untyped("tutorial.png"));
        preloaded.push(assets.load_untyped("wheat_growing.png"));
        preloaded.push(assets.load_untyped("wheat_grown.png"));
        preloaded.push(assets.load_untyped("wheat_unit.png"));
        preloaded.push(assets.load_untyped("fonts/ModeSeven.ttf"));

        cmd.insert_resource(Preload(preloaded));
    }

    fn in_game_clear_colour(mut clear_color: ResMut<ClearColor>) {
        clear_color.0 = Color::rgb_u8(17, 102, 0);
    }

    fn main_menu_clear_colour(mut clear_color: ResMut<ClearColor>) {
        clear_color.0 = Color::rgb(0.4, 0.4, 0.4);
    }

    fn pause_on_lost_focus(mut time: ResMut<Time>, windows: Res<Windows>) {
        if windows.primary().is_focused() {
            time.unpause();
        } else {
            time.pause();
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::preload)
            .init_resource::<MousePosition>()
            .init_resource::<Events<Reset>>()
            .add_event::<PlaySound>()
            .add_exit_system(GameState::InGame, Self::reset)
            .add_enter_system(GameState::InGame, Self::in_game_clear_colour)
            .add_enter_system(GameState::MainMenu, Self::main_menu_clear_colour)
            .add_system(Self::pause_on_lost_focus)
            .add_system(Self::update_bar.run_in_state(GameState::InGame))
            .add_system(Self::play_sound.run_in_state(GameState::InGame))
            .add_system(Self::update_mouse_position.run_in_state(GameState::InGame))
            .add_system(Self::handle_dead.run_in_state(GameState::InGame));
    }
}
