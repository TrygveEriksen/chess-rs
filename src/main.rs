use chess::ChessState;

mod chess;
mod move_generation;

fn main() {
    //let board: chess::ChessState = chess::ChessState::new();
    let fen = "8/2k2n2/8/3K1Q2/4NR2/8/8/8 w - - 0 1";
    let board: ChessState = chess::ChessState::from_fen(&fen);
    println!("FEN is: {}", fen);
    for idx in 0..8 {
        println!("{:?}", board.board[7 - idx])
    }
    let all_moves = ChessState::get_all_possible_moves(board);
    println!("All moves: {:?}", all_moves);
    println!("Num moves: {}", all_moves.len());
}
