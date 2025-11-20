#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}
#[derive(Debug)]
pub struct Queen {
    pub pos: ChessPosition,
}
impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            None
        } else {
            Some(ChessPosition { rank, file })
        }
    }
}
impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let a = &self.pos;
        let b = &other.pos;

        if a.rank == b.rank {
            return true;
        }

        if a.file == b.file {
            return true;
        }

        if (a.rank - b.rank).abs() == (a.file - b.file).abs() {
            return true;
        }

        false
    }
}