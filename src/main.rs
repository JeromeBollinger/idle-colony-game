use crate::game_camera::*;
use crate::player::*;
use crate::map::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod game_camera;
mod player;
mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Idle Colony Game".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(TilemapPlugin)
        .add_startup_system(initiate_map)
        .add_startup_system(default_camera)
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        .add_system(exit_game)
        .run();
}
