use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

mod consts;
mod game;
mod game_menu;
mod health;
mod main_menu;
mod plot;
mod selection;
mod unit;
mod utils;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum GameState {
    MainMenu,
    InGame,
}

#[derive(Component)]
pub struct MainCamera;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_loopless_state(GameState::MainMenu)
        .add_plugin(utils::Plugin)
        .add_plugin(main_menu::Plugin)
        .add_plugin(plot::Plugin)
        .add_plugin(unit::Plugin)
        .add_plugin(game_menu::Plugin)
        .add_plugin(health::Plugin)
        .add_plugin(game::Plugin)
        .add_plugin(selection::Plugin)
        .add_startup_system(init);

    app.run();
}

fn init(mut cmd: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.25;
    cmd.spawn((camera, MainCamera));
}
