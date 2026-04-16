use std::{collections::HashMap, fmt};
use owo_colors::{OwoColorize, Style};
use crate::{board::{Board, BoardType, Coord, Square}, player::{Player, PlayerID}};

pub enum GameAction {
    PlaceTile { player: PlayerID, coord: Coord }
}

pub enum GameResp {
    GameWinner { winner: PlayerID },
    GameDraw,
    TurnCompleted
}

pub enum InvalidAction {
    NotCurrentPlayer { current_player: PlayerID },
    InvalidPlacement { msg: String }
}

pub struct GameState {
    board: Board,
    player_order: Vec<PlayerID>,
    player_idx: usize,
    history: Vec<GameAction>,
    winner: Option<PlayerID>
}

impl GameState {

    pub fn new(board_size: BoardType, players: Vec<Player>) -> Self {

        let player_order: Vec<PlayerID> = players.iter().map(|p| p.get_id()).collect(); 
        let board = Board::new(board_size, player_order.clone());
        let history: Vec<GameAction> = Vec::new();

        Self { board, player_order, player_idx: 0, history, winner: None }
    }

    fn execute(&mut self, action: &GameAction) -> Result<(), InvalidAction>  {
        match action {
            GameAction::PlaceTile { player, coord } => {

                if self.current_player() != *player {
                    return Err(InvalidAction::NotCurrentPlayer { current_player: self.current_player() })
                }
                
                match self.board.place_tile(coord, *player) {
                    Ok(()) => {
                        if self.board.is_win(*player) {
                            self.winner = Some(*player);
                        }
                        Ok(())
                    },
                    Err(s) => Err(InvalidAction::InvalidPlacement { msg: s })
                }
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

    fn check_game_end_conditions(&mut self) -> Option<GameResp> {
        if self.board.is_win(self.current_player()) {
            self.winner = Some(self.current_player());
            Some(GameResp::GameWinner { winner: self.current_player() })
        } else if self.board.is_draw() {
            Some(GameResp::GameDraw)
        } else {
            None
        }
    }

    pub fn dispatch(&mut self, action: GameAction) -> Result<GameResp, InvalidAction> {

        match self.execute(&action) {
            Ok(()) => {
                self.update_history(action);
                if let Some(gr) = self.check_game_end_conditions() {
                    return Ok(gr)
                }
                self.next_player();
                println!("{}", self);
                return Ok(GameResp::TurnCompleted)
            },
            Err(e) => {
                println!("{}", self);
                return Err(e);
            }
        }

    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut potential_styles: Vec<Style> = vec![
            Style::new().red(),
            Style::new().green(),
            Style::new().purple(),
            Style::new().blue(),
            Style::new().yellow(),
            Style::new().cyan()
        ];

        let player_to_style: HashMap<PlayerID, Style> = self.player_order.iter().map(|p| {
            match potential_styles.pop() {
                Some(s) => (*p, s),
                None => (*p, Style::new().white())
            }
        }).collect();

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
                                let p_style = match player_to_style.get(&player_id) {
                                    Some(p) => *p,
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