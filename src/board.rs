use crate::tiles::Tile;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Board([Tile; 9]);

impl Default for Board {
    fn default() -> Self {
        Self([Tile::default(); 9])
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_string().fmt(f)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res: String = String::with_capacity(21);
        for r in 0..3 {
            for c in 0..3 {
                let pos = 3 * r + c;
                if self.0[pos] == Tile::X || self.0[pos] == Tile::O {
                    res.push_str(&self.0[pos].to_string())
                } else {
                    res.push_str(&pos.to_string())
                }
                res.push('|');
            }
            res.pop();
            res.push('\n');
            res.push_str("-+-+-");
            res.push('\n');
        }
        for _ in 0..6 {
            res.pop();
        }
        f.write_str(&res)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    InvalidPosition(String),
    InvalidEntry(String),
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn put(&mut self, position: usize, tile: Tile) -> Result<(), Error> {
        if position >= self.0.len() {
            return Err(Error::InvalidPosition(format!("Position, {position} is out of bounds, the position should be in range [0, 8]")));
        }
        if self.0[position] != Tile::Empty {
            return Err(Error::InvalidEntry(format!("Position {position} is already taken by Player {:?}", self.0[position])));
        }
        self.0[position] = tile;
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        self.0.chunks_exact(3).any(|c| c[0] == c[1] && c[1] == c[2] && c[0] != Tile::Empty) //checking for a row win
            || (0..3) //checking for a column win
                .any(|p| self.0[p] == self.0[p + 3] && self.0[p + 3] == self.0[p + 6] && self.0[p] != Tile::Empty) 
            || self.0[0] == self.0[4] && self.0[4] == self.0[8] && self.0[0] != Tile::Empty //backward diagonal
            || self.0[2] == self.0[4] && self.0[4] == self.0[6] && self.0[2] != Tile::Empty //forward diagonal
    }
}

#[cfg(test)] 
mod tests {
    use super::Board;
    use super::Tile;
    #[test]
    fn test_empty() {
        assert!(!Board::new().is_complete())
    }

    #[test]
    fn test_rows() {
        let mut board = Board::new();
        board.put(0, Tile::X).unwrap();
        board.put(1, Tile::X).unwrap();
        board.put(2, Tile::X).unwrap();
        assert!(board.is_complete());
    }

    #[test]
    fn test_cols() {
        let mut board = Board::new();
        board.put(1, Tile::O).unwrap();
        board.put(4, Tile::O).unwrap();
        board.put(7, Tile::O).unwrap();
    }

    #[test]
    fn test_diags() {
        let mut board = Board::new();
        board.put(1, Tile::O).unwrap();
        board.put(4, Tile::O).unwrap();
        board.put(7, Tile::O).unwrap();
        assert!(board.is_complete());

        board = Board::new();
        board.put(2, Tile::O).unwrap();
        board.put(4, Tile::O).unwrap();
        board.put(6, Tile::O).unwrap();
        assert!(board.is_complete());

        board = Board::new();
        board.put(0, Tile::O).unwrap();
        board.put(4, Tile::O).unwrap();
        board.put(8, Tile::O).unwrap();
        assert!(board.is_complete());
    }
}
