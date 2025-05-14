use crate::chess_init::ChessState;
const COLUMNS: &str = "abcdefgh";
const PIECE_INDICIES: &str = " PRNBQ";

fn slice_to_coord(col: char, row: char) -> (usize, usize) {
    (
        row.to_digit(10).unwrap() as u8 as usize - 1,
        (col as u8 - 97) as usize,
    )
}

impl ChessState {
    pub fn do_move(&self, _move: &String) -> ChessState {
        //Assumes the move is already safe
        let coefficient = if self.turn { 1 } else { -1 };
        let mut new_state: ChessState = self.copy();
        let m: String = _move.clone();
        let back_rank: usize = if new_state.turn { 0 } else { 7 };
        let mut ep: usize = 8;
        //castling
        if m == "O-O-O" {
            new_state.board[back_rank][0] = 0;
            new_state.board[back_rank][4] = 0;
            new_state.board[back_rank][2] = 6 * coefficient;
            new_state.board[back_rank][3] = 2 * coefficient;

            new_state.castling = if new_state.turn {
                new_state
                    .castling
                    .chars()
                    .filter(|x| x.is_ascii_lowercase())
                    .collect()
            } else {
                {
                    new_state
                        .castling
                        .chars()
                        .filter(|x| x.is_ascii_uppercase())
                        .collect()
                }
            }
        } else if m == "O-O" {
            new_state.board[back_rank][7] = 0;
            new_state.board[back_rank][4] = 0;
            new_state.board[back_rank][6] = 6 * coefficient;
            new_state.board[back_rank][5] = 2 * coefficient;
            new_state.castling = if new_state.turn {
                new_state
                    .castling
                    .chars()
                    .filter(|x| x.is_ascii_lowercase())
                    .collect()
            } else {
                {
                    new_state
                        .castling
                        .chars()
                        .filter(|x| x.is_ascii_uppercase())
                        .collect()
                }
            }
        }
        //pawn moves
        else if COLUMNS.contains(m.chars().nth(0).unwrap()) {
            let source: (usize, usize) =
                slice_to_coord(m.chars().nth(0).unwrap(), m.chars().nth(1).unwrap());
            let target: (usize, usize) =
                slice_to_coord(m.chars().nth(3).unwrap(), m.chars().nth(4).unwrap());

            if m.len() == 5 {
                if m.contains("x") && new_state.board[target.0][target.1] == 0 {
                    //en passant capture
                    new_state.board[(target.0 as i8 - coefficient) as usize][target.1] = 0;
                }

                new_state.board[target.0][target.1] = 1 * coefficient;

                if (source.0 as i8 - target.0 as i8) == 2 {
                    ep = source.1;
                }
            }
            //
            else {
                if m.contains("x") && new_state.board[target.0][target.1] == 0 {
                    //en passant capture
                    new_state.board[(target.0 as i8 - coefficient) as usize][target.1] = 0;
                }
                new_state.board[target.0][target.1] =
                    PIECE_INDICIES.find(m.chars().nth(5).unwrap()).unwrap() as u8 as i8
                        * coefficient;
            }
            new_state.board[source.0][source.1] = 0;
        }
        //all other moves
        else {
            let source: (usize, usize) =
                slice_to_coord(m.chars().nth(1).unwrap(), m.chars().nth(2).unwrap());
            let target: (usize, usize) =
                slice_to_coord(m.chars().nth(4).unwrap(), m.chars().nth(5).unwrap());
            let piece = new_state.board[source.0][source.1];
            new_state.board[source.0][source.1] = 0;
            new_state.board[target.0][target.1] = piece;
            new_state.en_passant = [0, 0, 0, 0, 0, 0, 0, 0];

            //Handle removing of castling opportunities
            if piece * coefficient == 6 {
                new_state.castling = if new_state.turn {
                    new_state
                        .castling
                        .chars()
                        .filter(|x| x.is_ascii_lowercase())
                        .collect()
                } else {
                    {
                        new_state
                            .castling
                            .chars()
                            .filter(|x| x.is_ascii_uppercase())
                            .collect()
                    }
                }
            } else if piece * coefficient == 4 {
                new_state.castling = if new_state.turn {
                    new_state
                        .castling
                        .chars()
                        .filter(|x| {
                            x.is_ascii_lowercase()
                                || x.to_string() == {
                                    if source == (back_rank, 0) {
                                        "K"
                                    } else if source == (back_rank, 7) {
                                        "Q"
                                    } else {
                                        ""
                                    }
                                }
                        })
                        .collect()
                } else {
                    {
                        new_state
                            .castling
                            .chars()
                            .filter(|x| {
                                x.is_ascii_uppercase()
                                    || x.to_string() == {
                                        if source == (back_rank, 0) {
                                            "k"
                                        } else if source == (back_rank, 7) {
                                            "q"
                                        } else {
                                            ""
                                        }
                                    }
                            })
                            .collect()
                    }
                }
            }
        }
        if ep != 8 {
            new_state.en_passant[ep] = 1;
        }

        new_state.turn = !new_state.turn;
        return new_state;
    }
}
