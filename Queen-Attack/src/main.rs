#[derive(Debug)]
pub struct ChessPosition {
    coords: (i32, i32),
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            Some(ChessPosition { coords: (rank, file) })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let delta = (
            self.pos.coords.0 - other.pos.coords.0,
            self.pos.coords.1 - other.pos.coords.1
        );

        delta.0 == 0 || delta.1 == 0 || delta.0.abs() == delta.1.abs()
    }
}