use crate::player::{PlayerID};

pub type BoardType = i8;
pub type Coord = (BoardType, BoardType);

#[derive(Clone, Copy)]
pub enum Square {
    Occupied(PlayerID),
    Empty
}

pub struct Board {
    grid: Vec<Square>,
    size: BoardType
}

impl Board {
    pub fn new(size: BoardType) -> Self {
        Self {
            grid: vec![Square::Empty; (size as usize).pow(2)],
            size: size
        }
    }

    fn outside_bounds(&self, coord: &Coord) -> bool {
        return (coord.0 < 0 || coord.0 > self.size) || (coord.1 < 0 || coord.1 > self.size)
    }

    pub fn get_tile(&self, coord: &Coord) -> Result<Square, String> {
        if self.outside_bounds(coord) {
            return Err("Outside of bounds".to_string())
        }
        Ok(self.grid[((coord.0 * self.size) + coord.1) as usize])
    }

    pub fn place_tile(&mut self, coord: &Coord, player_id: PlayerID) -> Result<(), String> {
        if self.outside_bounds(coord) {
            return Err("Outside of bounds".to_string())
        }

        self.grid[((coord.0 * self.size) + coord.1) as usize] = Square::Occupied(player_id);
        Ok(())
    }

    pub fn get_size(&self) -> &BoardType {
        &self.size
    }
}