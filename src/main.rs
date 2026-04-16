mod board;
mod game;
mod player;

use crate::{game::{GameAction, GameState, GameResp, InvalidAction}, player::Player};

fn main() {
    play_n_player_game(3)
}

fn generate_test_actions(num_players: i8) -> Vec<GameAction> {
    (0..6).into_iter().map(|y| {
        (0..num_players).into_iter().map(|x| {
            // GameAction::PlaceTile { player: x, coord: (x, y) } // horizontal
            // GameAction::PlaceTile { player: x, coord: (y, x) } // vertical
            // GameAction::PlaceTile { player: x, coord: (y, y) } // \ diagonal
            GameAction::PlaceTile { player: x, coord: (5 - y + x, y) } // / diagonal
        }).collect::<Vec<GameAction>>()
    }).flatten().collect()
}

fn play_n_player_game(num_players: i8) {

    let players = (0..num_players).into_iter().map(|idx| Player::new(idx)).collect();
    
    let mut game = GameState::new(11, players);
    let actions = generate_test_actions(num_players);

    println!("{}", game);

    for action in actions {
        match game.dispatch(action) {
            Ok(r) => {
                match r {
                    GameResp::GameWinner { winner } => panic!("{} has won", winner),
                    GameResp::GameDraw => panic!("Game drawn"),
                    _ => {}
                }
            },
            Err(e) => {
                match e {
                    InvalidAction::NotCurrentPlayer { current_player } => {
                        panic!("It is player {} turn", current_player)
                    },
                    InvalidAction::InvalidPlacement { msg } => {
                        panic!("{}", msg)
                    }
                }
            }
        };
    }
}