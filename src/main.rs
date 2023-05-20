use crate::game_camera::*;
use crate::player::*;
use bevy::prelude::*;

mod game_camera;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Idle Colony Game".into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(default_camera)
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        .add_system(exit_game)
        .add_system(camera_zoom)
        .run();
}
