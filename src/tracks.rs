use bevy::prelude::*;
use std::fmt;

#[derive(Debug)]
pub enum TrackKind {
    Horizontal,
    Vertical,
    Cross,
}

#[derive(Debug, Component)]
pub struct Track {
    kind: TrackKind,
}

impl Track {
    pub fn new(kind: TrackKind) -> Self {
        Track{
            kind,
        }
    }

    pub fn default() -> Self {
        Track{
            kind: TrackKind::Cross,
        }
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "kind={}", self.kind)
    }
}

impl fmt::Display for TrackKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Horizontal => write!(f, "kind=Horizontal"),
            Self::Vertical => write!(f, "kind=Vertical"),
            Self::Cross => write!(f, "kind=Cross"),
        }
    }
}

pub fn spawn_tracks(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            texture: asset_server.load("player.png"),
            ..default()
        },
        Track::default(),
    ));
}
