use std::{collections::HashMap, fmt};
use owo_colors::{OwoColorize, Style};
use crate::{board::{Board, BoardType, Coord, Square}, player::{Player, PlayerID}};

pub enum GameAction {
    PlaceTile { player: PlayerID, coord: Coord }
}

pub struct GameState {
    board: Board,
    players: HashMap<PlayerID, Player>,
    player_order: Vec<PlayerID>,
    player_idx: usize,
    history: Vec<GameAction>
}

impl GameState {

    pub fn new(board_size: BoardType, players: Vec<Player>) -> Self {

        let board = Board::new(board_size);
        let player_order = players.iter().map(|p| p.get_id()).collect(); 
        let players = players.into_iter().map(|p| (p.get_id(), p)).collect();
        let history = Vec::new();

        Self { board, players, player_order, player_idx: 0, history }
    }

    fn execute(&mut self, action: &GameAction) -> Result<(), String>  {
        match action {
            GameAction::PlaceTile { player, coord } => {
                
                if self.current_player() != *player {
                    return Err(format!("It is Player {} turn", self.current_player()))
                }

                return self.board.place_tile(coord, *player);
            }
        }
    }

    fn current_player(&self) -> PlayerID {
        self.player_order[self.player_idx]
    }

    fn update_history(&mut self, action: GameAction) {
        self.history.push(action);
    }

    fn next_player(&mut self) {
        self.player_idx = (self.player_idx + 1) % self.player_order.len()
    }

    pub fn dispatch(&mut self, action: GameAction) -> Result<(), String> {

        match self.execute(&action) {
            Ok(()) => {
                self.update_history(action);
                self.next_player();
                println!("{}", self);
                return Ok(());
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.board.get_size();

        for x in 0..*size {
            for y in 0..*size {
                let c: Coord = (x, y);
                match self.board.get_tile(&c) {
                    Ok(s) => {
                        match s {
                            Square::Empty => {
                                write!(f, "o").expect("Not Written");
                            },
                            Square::Occupied(player_id) => {
                                let p_style = match self.players.get(&player_id) {
                                    Some(p) => p.get_style(),
                                    None => Style::new().black()
                                };
                                write!(f, "{}", "X".style(p_style)).expect("Not Written");
                            }
                        }
                    },
                    Err(_) => {}
                }
            }
            write!(f, "\n").expect("Not Written");
        }
        Ok(())
    }
}