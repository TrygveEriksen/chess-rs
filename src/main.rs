use std::io;

use chess_init::ChessState;
use chessbot::{ChessBot, MinimaxBot};
mod chess_init;
mod chessbot;
mod eval;
mod move_generation;
mod moves;
mod translator;

fn main() {
    let mut message: String = String::new();
    let bot: MinimaxBot = MinimaxBot {
        player: true,
        max_depth: 3,
    };
    let mut state: ChessState = ChessState::new_board();

    loop {
        io::stdin()
            .read_line(&mut message)
            .expect("Something went wrong with reading stdin");
        match message
            .as_str()
            .trim()
            .split(|x: char| x.is_ascii_whitespace())
            .nth(0)
            .expect("Split error")
        {
            "uci" => {
                println!("uciok")
            }
            "isready" => {
                println!("readyok")
            }
            "ucinewgame" => {
                state = ChessState::new_board();
            }
            "position" => {
                if message.contains("startpos") {
                    state = ChessState::new_board();
                } else if message.contains("fen") {
                    state = ChessState::from_fen(
                        message
                            .as_str()
                            .trim()
                            .split(|x: char| x.is_ascii_whitespace())
                            .nth(2)
                            .expect("Split error"),
                    );
                }
                if message.contains("moves") {
                    let moves: Vec<String> = message
                        .as_str()
                        .trim()
                        .split(|x: char| x.is_ascii_whitespace())
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .clone();
                    let idx = moves
                        .iter()
                        .position(|x| x == "moves")
                        .expect("Moves not found");
                    for m in moves[idx + 1..].iter() {
                        state = state.do_move(&translator::uci_to_lan(&state, &m.to_string()));
                    }
                }
            }
            "go" => {
                let m = bot.respond(state.copy());
                println!("bestmove {}", translator::lan_to_uci(&state, &m));
                state = state.do_move(&m);
            }
            "stop" => {
                break;
            }
            "quit" => {
                break;
            }
            _ => {}
        }
        message = String::new();
    }
}
