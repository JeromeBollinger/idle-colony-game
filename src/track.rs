use bevy::prelude::*;

#[derive(Component)]
pub struct Track {
    track_type: TrackType,
}

impl Track {
    pub fn new() -> Self{
        Track{
            track_type: TrackType::Horizontal
        }
    }
}

enum TrackType {
    Horizontal,
    Vertical,
    Cross
}

pub fn spawn_track(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(8.0, 8.0, 2.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(1.0, 1.0, 1.0),
            },
            texture: asset_server.load("track_horizontal.png"),
            ..default()
        },
        Track::new(),
    ));
}
