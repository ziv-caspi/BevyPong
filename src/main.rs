mod ball;
mod border;
mod game_manager;
mod game_text;
mod paddle;
mod utils;

use ball::BallPlugin;
use bevy::prelude::*;
use border::BordersPlugin;
use game_manager::GameManagerPlugin;
use game_text::GameTextPlugin;
use paddle::PaddlesPlugin;

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            BallPlugin,
            PaddlesPlugin,
            BordersPlugin,
            GameManagerPlugin,
            GameTextPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}
