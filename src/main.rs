use bevy::prelude::*;
use iyes_loopless::prelude::*;

mod consts;
mod game;
mod main_menu;

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
        .add_loopless_state(GameState::MainMenu)
        .add_plugin(main_menu::Plugin)
        .add_plugin(game::Plugin)
        .add_startup_system(init);

    app.run();
}

fn init(mut cmd: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.25;
    cmd.spawn((camera, MainCamera));
}
