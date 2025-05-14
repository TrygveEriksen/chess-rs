use std::str;
const BLACK_PIECE_INDICIES: &str = " prnbqk";
const WHITE_PIECE_INDICIES: &str = " PRNBQK";

pub struct ChessState {
    pub board: [[i8; 8]; 8],
    pub turn: bool,
    pub en_passant: [u8; 8],
    pub castling: String,
    pub halfmoves: i8,
    pub fullmoves: i8,
}

impl ChessState {
    pub fn new_board() -> ChessState {
        return ChessState {
            board: [
                [2, 3, 4, 5, 6, 4, 3, 2],
                [1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-2, -3, -4, -5, -6, -4, -3, -2],
            ],
            en_passant: [0, 0, 0, 0, 0, 0, 0, 0],
            turn: true,
            castling: "kqKQ".to_string(),
            halfmoves: 0,
            fullmoves: 0,
        };
    }
    fn empty_board() -> ChessState {
        return ChessState {
            board: [
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ],
            en_passant: [0, 0, 0, 0, 0, 0, 0, 0],
            turn: true,
            castling: "kqKQ".to_string(),
            halfmoves: 0,
            fullmoves: 0,
        };
    }

    pub fn from_fen(fen_string: &str) -> ChessState {
        let mut board: ChessState = ChessState::empty_board();
        let fen_string_parts: Vec<&str> = fen_string.split(" ").collect();
        ChessState::board_from_fen(&mut board.board, fen_string_parts[0]);
        board.turn = fen_string_parts[1] == "w";
        board.castling = fen_string_parts[2].to_string();
        ChessState::en_passant_from_fen(&mut board.en_passant, fen_string_parts[3]);
        board.halfmoves = fen_string_parts[4].parse::<i8>().unwrap();
        board.fullmoves = fen_string_parts[5].parse::<i8>().unwrap();
        return board;
    }

    fn board_from_fen(board: &mut [[i8; 8]; 8], fen_string: &str) {
        let rows = fen_string.split("/");
        for (idx, row) in rows.into_iter().enumerate() {
            let mut jdx: usize = 0;
            for s in row.chars() {
                if WHITE_PIECE_INDICIES.contains(s) {
                    board[7 - idx][jdx] = WHITE_PIECE_INDICIES
                        .find(s)
                        .expect("Error in parsing FEN string")
                        as i8;
                    jdx += 1;
                }
                if BLACK_PIECE_INDICIES.contains(s) {
                    board[7 - idx][jdx] = -1
                        * BLACK_PIECE_INDICIES
                            .find(s)
                            .expect("Error in parsing FEN string") as i8;
                    jdx += 1;
                }
                if s.is_ascii_digit() {
                    jdx += s.to_digit(10).expect("Error in parsing FEN string") as usize;
                }
            }
        }
    }

    fn en_passant_from_fen(en_passant_field: &mut [u8], en_passant_str: &str) {
        if en_passant_str == "-" {
            return;
        }
        let pos: usize = (en_passant_str.chars().nth(0).unwrap() as u8 - 97) as usize;
        en_passant_field[pos] = 1;
    }

    pub fn copy(&self) -> ChessState {
        ChessState {
            board: self.board.clone(),
            turn: self.turn.clone(),
            en_passant: self.en_passant.clone(),
            castling: self.castling.clone(),
            halfmoves: self.halfmoves.clone(),
            fullmoves: self.fullmoves.clone(),
        }
    }
}
