use crate::game_camera::*;
use crate::map::*;
use crate::player::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use tracks::*;
use wasm_bindgen::prelude::*;

mod game_camera;
mod map;
mod player;
mod tracks;

#[wasm_bindgen]
pub fn run() {
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
        .add_startup_system(spawn_tracks)
        .add_startup_system(default_camera)
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        .add_system(exit_game)
        .add_system(camera_zoom)
        .run();
}
