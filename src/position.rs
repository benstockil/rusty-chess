use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct BoardPosition {
    pub rank: u8,
    pub file: u8,
    // pub index: u8,
}

impl Display for BoardPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = (b'a' + self.file) as char;
        let rank = self.rank + 1;

        write!(f, "{}{}", file, rank)
    }
}

impl BoardPosition {
    pub fn from_rank_file(rank: u8, file: u8) -> Self {
        Self { rank, file }
    }

    pub fn from_index(index: u8) -> Self {
        Self {
            rank: index / 8,
            file: index % 8,
        }
    }

    // pub fn rank(&self) -> u8 {
    //     self.index / 8
    // }
    //
    // pub fn file(&self) -> u8 {
    //     self.index % 8
    // }
    pub fn rank(&self) -> u8 {
        self.rank
    }

    pub fn file(&self) -> u8 {
        self.file
    }

    pub fn index(&self) -> u8 {
        self.rank * 8 + self.file
    }
}

impl From<(u8, u8)> for BoardPosition {
    fn from(coords: (u8, u8)) -> Self {
        Self::from_rank_file(coords.0, coords.1)
    }
}

impl From<u8> for BoardPosition {
    fn from(index: u8) -> Self {
        Self::from_index(index)
    }
}
