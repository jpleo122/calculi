mod board;
mod game;
mod player;
use owo_colors::{Style};

use crate::{game::{GameAction, GameState}, player::Player};

fn main() {

    let players = vec![
        Player::new(0, Style::new().red()),
        Player::new(1, Style::new().green())
    ];
    
    let mut game = GameState::new(10, players);

    let actions = vec![
        GameAction::PlaceTile { player: 0, coord: (0, 0) },
        GameAction::PlaceTile { player: 1, coord: (1, 0) },
        GameAction::PlaceTile { player: 0, coord: (0, 1) },
        GameAction::PlaceTile { player: 1, coord: (1, 1) },
        GameAction::PlaceTile { player: 0, coord: (0, 2) },
        GameAction::PlaceTile { player: 1, coord: (1, 2) },
        GameAction::PlaceTile { player: 0, coord: (0, 3) },
        GameAction::PlaceTile { player: 1, coord: (1, 3) },
        GameAction::PlaceTile { player: 0, coord: (0, 4) },
    ];

    println!("{}", game);

    for action in actions {
        match game.dispatch(action) {
            Err(e) => {
                panic!("{}", e)
            },
            _ => {}
        };
    }
}
