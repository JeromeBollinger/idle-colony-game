use bevy::prelude::*;
use std::fmt;

#[derive(Debug)]
pub enum TrackKind {
    Horizontal,
    Vertical,
    Cross,
    CurveTR,
    CurveTL,
    CurveBR,
    CurveBL,
    TLeft,
    TRight,
    TBottom,
    TTop,
}

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
            Self::CurveTR => write!(f, "kind=CurveTR"),
            Self::CurveTL => write!(f, "kind=CurveTL"),
            Self::CurveBR => write!(f, "kind=CurveBR"),
            Self::CurveBL => write!(f, "kind=CurveBL"),
            Self::TLeft => write!(f, "kind=TLeft"),
            Self::TRight => write!(f, "kind=TRight"),
            Self::TBottom => write!(f, "kind=TBottom"),
            Self::TTop => write!(f, "kind=TTop"),
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
