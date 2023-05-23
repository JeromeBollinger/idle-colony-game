use std::fmt;

pub enum TrackKind {
    horizontal,
    vertical,
    cross,
}

#[derive(Debug)]
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
            kind: TrackKind::cross,
        }
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "kind={}", self.kind)
    }
}
