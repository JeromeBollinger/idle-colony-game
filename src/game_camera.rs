use bevy::prelude::*;

pub fn default_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}
