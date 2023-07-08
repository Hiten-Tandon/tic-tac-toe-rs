#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash, PartialEq)]
#[repr(u8)]
pub enum Tile {
    Empty,
    X,
    O,
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Self::Empty => String::from(" "),
            Self::O => String::from("O"),
            Self::X => String::from("X"),
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}

impl Tile {
    pub fn switch(self) -> Self {
        match self {
            Tile::Empty => Tile::Empty,
            Tile::X => Tile::O,
            Tile::O => Tile::X,
        }
    }
}
