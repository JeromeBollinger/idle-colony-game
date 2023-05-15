use bevy::prelude::*;
use crate::game_camera::*;
mod game_camera;
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
        .run();
}
