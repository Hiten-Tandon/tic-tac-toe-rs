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

impl Board {
    pub fn new() -> Self {
        Self::default()
    }
}
