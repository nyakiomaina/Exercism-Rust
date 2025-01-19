// #[derive(Debug)]
// pub struct ChessPosition {
//     coords: (i32, i32),
// }

// #[derive(Debug)]
// pub struct Queen {
//     pos: ChessPosition,
// }

// impl ChessPosition {
//     pub fn new(rank: i32, file: i32) -> Option<Self> {
//         if (0..8).contains(&rank) && (0..8).contains(&file) {
//             Some(ChessPosition { coords: (rank, file) })
//         } else {
//             None
//         }
//     }
// }

// impl Queen {
//     pub fn new(position: ChessPosition) -> Self {
//         Queen { pos: position }
//     }

//     pub fn can_attack(&self, other: &Queen) -> bool {
//         let delta = (
//             self.pos.coords.0 - other.pos.coords.0,
//             self.pos.coords.1 - other.pos.coords.1
//         );

//         delta.0 == 0 || delta.1 == 0 || delta.0.abs() == delta.1.abs()
//     }
// }

// position of the chessboard
#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

// queen on the chessboard
#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

//posible ways queen can attack
enum AttackType {
    Horizontal,
    Vertical,
    Diagonal,
    None,
}

//valide chess position
impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            Some(ChessPosition { rank, file })
        } else {
            None
        }
    }
}

impl queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        match self.get_attack_type(other) {
            attackType::None => false,
            _=> true,
        }
    }

    fn get_attack_type(&self, other: &Queen) -> AttackType {
        if self.position.rank == other.position.rank {
            AttackType::Horizontal
        }
        else if self.position.file == other.position.file {
            AttackType::Vertical
        }

        else if (self.position.rank - other.position.rank).abs() ==
            (self.position.rank - other.position.rank).abs() {
                AttackType::Diagonal
        }
        else {
            AttackType::None
        }
    }
}