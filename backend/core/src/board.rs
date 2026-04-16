use std::{collections::HashMap};

use crate::{player::PlayerID};

type BitType = i128;
pub type BoardType = i8;
pub type Coord = (BoardType, BoardType);

#[derive(Clone, Copy)]
pub enum Square {
    Occupied(PlayerID),
    Empty
}

pub struct Board {
    grid: HashMap<PlayerID, BitType>,
    size: BoardType,
    win_length: i8
}

impl Board {
    pub fn new(size: BoardType, players: Vec<PlayerID>) -> Self {
        let grid: HashMap<PlayerID, BitType> = players.into_iter().map(|p| (p, 0)).collect();
        if size > 11 { panic!("Using i128, board can't be bigger currently!")}
        Self {
            grid: grid,
            size: size,
            win_length: 4
        }
    }

    fn outside_bounds(&self, coord: &Coord) -> bool {
        return (coord.0 < 0 || coord.0 >= self.size) || (coord.1 < 0 || coord.1 >= self.size)
    }

    fn coord_to_bit(&self, coord: &Coord) -> BitType {
        1 << ((coord.0 * self.size) + coord.1)
    }

    fn is_occupied(&self, coord: &Coord) -> Option<PlayerID> {
        let coord_bit = self.coord_to_bit(coord);
        for (key, value) in self.grid.iter() {
            if (value & coord_bit) != 0 {
                return Some(*key);
            }
        }
        None
    }

    fn apply_bitwise_n_times(&self, board: BitType, offset: i8) -> bool {
        let mut y = board;
        for _ in 1..self.win_length {
            y = y & (y << offset);
        }
        y != 0
    }

    pub fn is_draw(&self) -> bool {
        let mut y = 0;
        for i in self.grid.values() {
            y = y | i;
        }
        y == BitType::MAX
    }
    
    pub fn is_win(&self, player_id: PlayerID) -> bool {
        let board = self.grid[&player_id];
        self.apply_bitwise_n_times(board, 1) || // horizontal win 
        self.apply_bitwise_n_times(board, self.size) || // vertical win
        self.apply_bitwise_n_times(board, self.size + 1) || // \ diagonal win
        self.apply_bitwise_n_times(board, self.size - 1) // / diagonal win
    }

    pub fn get_tile(&self, coord: &Coord) -> Result<Square, String> {
        if self.outside_bounds(coord) {
            return Err("Outside of bounds".to_string())
        }
        match self.is_occupied(coord) {
            Some(p) => Ok(Square::Occupied(p)),
            None => Ok(Square::Empty)
        }
    }

    pub fn place_tile(&mut self, coord: &Coord, player_id: PlayerID) -> Result<(), String> {
        if self.outside_bounds(coord) {
            return Err("Outside of bounds".to_string())
        }

        if let Some(_) = self.is_occupied(coord) {
            return Err("Square is occupied".to_string())
        }

        let coord_bit = self.coord_to_bit(coord);
        self.grid.insert(player_id, self.grid[&player_id] | coord_bit);
        Ok(())
    }

    pub fn get_size(&self) -> &BoardType {
        &self.size
    }
}