use std::{collections::HashSet, usize};

use crate::chess::ChessState;

const VERTICAL: [(i8, i8); 2] = [(1, 0), (-1, 0)];
const HORISONTAL: [(i8, i8); 2] = [(0, 1), (0, -1)]; //
const LEFT_DIAGONAL: [(i8, i8); 2] = [(1, -1), (-1, 1)]; //TODO:fix
const RIGHT_DIAGONAL: [(i8, i8); 2] = [(1, 1), (-1, -1)];
const KNIGHT_DIRECTIONS: [(i8, i8); 8] = [
    (1, 2),
    (-1, 2),
    (1, -2),
    (-1, -2),
    (2, -1),
    (2, 1),
    (-2, 1),
    (-2, -1),
];
const KING_DIRECTIONS: [(i8, i8); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, -1),
    (-1, 1),
    (1, 1),
    (-1, -1),
];
const STRAIGHT: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const DIAGONAL: [(i8, i8); 4] = [(1, -1), (-1, 1), (1, 1), (-1, -1)];
const PIECE_SYMBOLS: &'static [&'static str; 4] = &["R", "B", "N", "Q"];

impl ChessState {
    fn find_positions(board: [[i8; 8]; 8], piece: i8) -> Vec<(usize, usize)> {
        let mut positions: Vec<(usize, usize)> = Vec::new();
        for (idx, row) in board.iter().enumerate() {
            for (jdx, field) in row.iter().enumerate() {
                if field - piece == 0 {
                    positions.push((idx, jdx));
                }
            }
        }
        return positions;
    }
    fn find_position(board: [[i8; 8]; 8], piece: i8) -> (usize, usize) {
        for (idx, row) in board.iter().enumerate() {
            for (jdx, field) in row.iter().enumerate() {
                if field - piece == 0 {
                    return (idx, jdx);
                }
            }
        }
        return (9, 9);
    }

    pub fn get_all_possible_moves(state: ChessState) -> Vec<String> {
        let player: bool = state.turn;
        let coefficient: i8 = if player { 1 } else { -1 };

        let mut all_moves: Vec<String> = Vec::new();
        let p_pos: Vec<(usize, usize)> = ChessState::find_positions(state.board, 1 * coefficient);
        let r_pos: Vec<(usize, usize)> = ChessState::find_positions(state.board, 2 * coefficient);
        let n_pos: Vec<(usize, usize)> = ChessState::find_positions(state.board, 3 * coefficient);
        let b_pos: Vec<(usize, usize)> = ChessState::find_positions(state.board, 4 * coefficient);
        let q_pos: (usize, usize) = ChessState::find_position(state.board, 5 * coefficient);
        let k_pos: (usize, usize) = ChessState::find_position(state.board, 6 * coefficient);
        let horisontal_pin: Vec<(usize, usize)> =
            ChessState::horisontal_pin(state.board, k_pos, coefficient);
        let vertical_pin: Vec<(usize, usize)> =
            ChessState::vertical_pin(state.board, k_pos, coefficient);
        let left_diagonal_pin: Vec<(usize, usize)> =
            ChessState::left_diagonal_pin(state.board, k_pos, coefficient);
        let right_diagonal_pin: Vec<(usize, usize)> =
            ChessState::right_diagonal_pin(state.board, k_pos, coefficient);
        let danger_squares: HashSet<(usize, usize)> =
            ChessState::danger_squares(state.board, coefficient);
        if player {
            //white move generation
            if danger_squares.contains(&k_pos) {
            } else {
                //Pawns
                for pos in p_pos.iter() {
                    if state.board[pos.0 + 1][pos.1] == 0
                        && !(horisontal_pin.contains(pos)
                            || left_diagonal_pin.contains(pos)
                            || right_diagonal_pin.contains(pos))
                    {
                        let m = format!(
                            "{}{}-{}{}",
                            (pos.1 + 97) as u8 as char,
                            pos.0 + 1,
                            (pos.1 + 97) as u8 as char,
                            pos.0 + 2
                        );
                        if pos.0 == 1 && state.board[3][pos.1] == 0 {
                            //White double pawn move
                            all_moves.push(format!(
                                "{}{}-{}{}",
                                (pos.1 + 97) as u8 as char,
                                2,
                                (pos.1 + 97) as u8 as char,
                                4
                            ));
                        }
                        if pos.0 == 6 {
                            for piece in PIECE_SYMBOLS.iter() {
                                all_moves.push(m.clone() + piece);
                            }
                        } else {
                            all_moves.push(m);
                        }
                    }
                    if pos.1 > 0
                        && (state.board[pos.0 + 1][pos.1 - 1] < 0
                            || (state.en_passant[pos.1 - 1] != 0 && pos.0 == 4))
                        && !(vertical_pin.contains(pos)
                            || horisontal_pin.contains(pos)
                            || right_diagonal_pin.contains(pos))
                    {
                        let m = format!(
                            "{}{}x{}{}",
                            (pos.1 + 97) as u8 as char,
                            pos.0 + 1,
                            (pos.1 + 96) as u8 as char,
                            pos.0 + 2
                        );
                        if pos.0 == 6 {
                            for piece in PIECE_SYMBOLS.iter() {
                                all_moves.push(m.clone() + piece);
                            }
                        } else {
                            all_moves.push(m.clone());
                        }
                    }
                    if pos.1 < 7
                        && (state.board[pos.0 + 1][pos.1 + 1] < 0
                            || (state.en_passant[pos.1 + 1] != 0 && pos.0 == 4))
                        && !(vertical_pin.contains(pos)
                            || horisontal_pin.contains(pos)
                            || left_diagonal_pin.contains(pos))
                    {
                        let m = format!(
                            "{}{}x{}{}",
                            (pos.1 + 97) as u8 as char,
                            pos.0 + 1,
                            (pos.1 + 98) as u8 as char,
                            pos.0 + 2
                        );
                        if pos.0 == 6 {
                            for piece in PIECE_SYMBOLS.iter() {
                                all_moves.push(m.clone() + piece);
                            }
                        } else {
                            all_moves.push(m);
                        }
                    }
                }
                for pos in r_pos.iter() {
                    let mut possible: Vec<(usize, usize)> = Vec::new();
                    if !(vertical_pin.contains(pos)
                        || right_diagonal_pin.contains(pos)
                        || left_diagonal_pin.contains(pos))
                    {
                        //Horisontal rook moves
                        for dir in HORISONTAL {
                            possible.append(&mut ChessState::directional_moves(
                                state.board,
                                *pos,
                                dir,
                                player,
                            ));
                        }
                    }
                    if !(horisontal_pin.contains(pos)
                        || right_diagonal_pin.contains(pos)
                        || left_diagonal_pin.contains(pos))
                    {
                        //Vertical rook moves
                        for dir in VERTICAL {
                            possible.append(&mut ChessState::directional_moves(
                                state.board,
                                *pos,
                                dir,
                                player,
                            ));
                        }
                    }
                    for new_pos in possible.iter() {
                        all_moves.push(if state.board[new_pos.0][new_pos.1] == 0 {
                            format!(
                                "R{}{}-{}{}",
                                (pos.1 + 97) as u8 as char,
                                pos.0 + 1,
                                (new_pos.1 + 97) as u8 as char,
                                new_pos.0 + 1
                            )
                        } else {
                            format!(
                                "R{}{}x{}{}",
                                (pos.1 + 97) as u8 as char,
                                pos.0 + 1,
                                (new_pos.1 + 97) as u8 as char,
                                new_pos.0 + 1
                            )
                        });
                    }
                }
                for pos in b_pos.iter() {
                    let mut possible: Vec<(usize, usize)> = Vec::new();
                    if !(vertical_pin.contains(pos)
                        || horisontal_pin.contains(pos)
                        || left_diagonal_pin.contains(pos))
                    {
                        //Horisontal rook moves
                        for dir in RIGHT_DIAGONAL {
                            possible.append(&mut ChessState::directional_moves(
                                state.board,
                                *pos,
                                dir,
                                player,
                            ));
                        }
                    }
                    if !(vertical_pin.contains(pos)
                        || horisontal_pin.contains(pos)
                        || right_diagonal_pin.contains(pos))
                    {
                        //Vertical rook moves
                        for dir in LEFT_DIAGONAL {
                            possible.append(&mut ChessState::directional_moves(
                                state.board,
                                *pos,
                                dir,
                                player,
                            ));
                        }
                    }
                    for new_pos in possible.iter() {
                        all_moves.push(if state.board[new_pos.0][new_pos.1] == 0 {
                            format!(
                                "B{}{}-{}{}",
                                (pos.1 + 97) as u8 as char,
                                pos.0 + 1,
                                (new_pos.1 + 97) as u8 as char,
                                new_pos.0 + 1
                            )
                        } else {
                            format!(
                                "B{}{}x{}{}",
                                (pos.1 + 97) as u8 as char,
                                pos.0 + 1,
                                (new_pos.1 + 97) as u8 as char,
                                new_pos.0 + 1
                            )
                        });
                    }
                }
                for pos in n_pos.iter() {
                    if !(vertical_pin.contains(pos)
                        || horisontal_pin.contains(pos)
                        || left_diagonal_pin.contains(pos)
                        || right_diagonal_pin.contains(pos))
                    {
                        for dir in KNIGHT_DIRECTIONS {
                            let new_pos = (pos.0 as i8 + dir.0, pos.1 as i8 + dir.1);
                            if -1 < new_pos.0 && new_pos.0 < 8 && -1 < new_pos.1 && new_pos.1 < 7 {
                                if state.board[new_pos.0 as usize][new_pos.1 as usize] < 1 {
                                    all_moves.push(
                                        if state.board[new_pos.0 as usize][new_pos.1 as usize] == 0
                                        {
                                            format!(
                                                "N{}{}-{}{}",
                                                (pos.1 + 97) as u8 as char,
                                                pos.0 + 1,
                                                (new_pos.1 + 97) as u8 as char,
                                                new_pos.0 + 1
                                            )
                                        } else {
                                            format!(
                                                "N{}{}x{}{}",
                                                (pos.1 + 97) as u8 as char,
                                                pos.0 + 1,
                                                (new_pos.1 + 97) as u8 as char,
                                                new_pos.0 + 1
                                            )
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
                if q_pos.0 != 9 {
                    let mut possible: Vec<(usize, usize)> = Vec::new();
                    if !(vertical_pin.contains(&q_pos)
                        || horisontal_pin.contains(&q_pos)
                        || left_diagonal_pin.contains(&q_pos))
                    {
                        for dir in RIGHT_DIAGONAL {
                            possible.append(&mut ChessState::directional_moves(
                                state.board,
                                q_pos,
                                dir,
                                player,
                            ));
                        }
                    }

                    if !(vertical_pin.contains(&q_pos)
                        || horisontal_pin.contains(&q_pos)
                        || right_diagonal_pin.contains(&q_pos))
                    {
                        for dir in LEFT_DIAGONAL {
                            possible.append(&mut ChessState::directional_moves(
                                state.board,
                                q_pos,
                                dir,
                                player,
                            ));
                        }
                    }

                    if !(vertical_pin.contains(&q_pos)
                        || right_diagonal_pin.contains(&q_pos)
                        || left_diagonal_pin.contains(&q_pos))
                    {
                        for dir in HORISONTAL {
                            possible.append(&mut ChessState::directional_moves(
                                state.board,
                                q_pos,
                                dir,
                                player,
                            ));
                        }
                    }

                    if !(right_diagonal_pin.contains(&q_pos)
                        || horisontal_pin.contains(&q_pos)
                        || left_diagonal_pin.contains(&q_pos))
                    {
                        {
                            for dir in VERTICAL {
                                possible.append(&mut ChessState::directional_moves(
                                    state.board,
                                    q_pos,
                                    dir,
                                    player,
                                ))
                            }
                        }
                    }
                    for new_pos in possible.iter() {
                        all_moves.push(if state.board[new_pos.0][new_pos.1] == 0 {
                            format!(
                                "Q{}{}-{}{}",
                                (q_pos.1 + 97) as u8 as char,
                                q_pos.0 + 1,
                                (new_pos.1 + 97) as u8 as char,
                                new_pos.0 + 1
                            )
                        } else {
                            format!(
                                "Q{}{}x{}{}",
                                (q_pos.1 + 97) as u8 as char,
                                q_pos.0 + 1,
                                (new_pos.1 + 97) as u8 as char,
                                new_pos.0 + 1
                            )
                        });
                    }
                }
                if k_pos.0 != 9 {
                    for dir in KING_DIRECTIONS {
                        let new_pos: (i8, i8) = (k_pos.0 as i8 + dir.0, k_pos.1 as i8 + dir.1);
                        if -1 < new_pos.0
                            && new_pos.0 < 8
                            && -1 < new_pos.1
                            && new_pos.1 < 8
                            && !danger_squares.contains(&(new_pos.0 as usize, new_pos.1 as usize))
                        {
                            all_moves.push(
                                if state.board[new_pos.0 as usize][new_pos.1 as usize] == 0 {
                                    format!(
                                        "K{}{}-{}{}",
                                        (k_pos.1 + 97) as u8 as char,
                                        k_pos.0 + 1,
                                        (new_pos.1 + 97) as u8 as char,
                                        new_pos.0 + 1
                                    )
                                } else {
                                    format!(
                                        "K{}{}x{}{}",
                                        (k_pos.1 + 97) as u8 as char,
                                        k_pos.0 + 1,
                                        (new_pos.1 + 97) as u8 as char,
                                        new_pos.0 + 1
                                    )
                                },
                            );
                        }
                    }
                    // White castling
                    if state.castling.contains("K")
                        && state.board[0][5] == 0
                        && state.board[0][6] == 0
                        && !danger_squares.contains(&(0, 5))
                        && !danger_squares.contains(&(0, 6))
                    {
                        all_moves.push("O-O".into())
                    }
                    if state.castling.contains("Q")
                        && state.board[0][1] == 0
                        && state.board[0][2] == 0
                        && state.board[0][3] == 0
                        && !danger_squares.contains(&(0, 1))
                        && !danger_squares.contains(&(0, 2))
                        && !danger_squares.contains(&(0, 3))
                    {
                        all_moves.push("O-O-O".into())
                    }
                }
            }
        }
        //L
        //L
        //L
        //L
        //L
        //L
        //L
        //L
        //L
        //L
        //L
        else {
            //Black move generation
            if danger_squares.contains(&k_pos) {
            } else {
                //Pawns
                for pos in p_pos.iter() {
                    if state.board[pos.0 - 1][pos.1] == 0
                        && !(horisontal_pin.contains(pos)
                            || left_diagonal_pin.contains(pos)
                            || right_diagonal_pin.contains(pos))
                    {
                        let m = format!(
                            "{}{}-{}{}",
                            (pos.1 + 97) as u8 as char,
                            pos.0 + 1,
                            (pos.1 + 97) as u8 as char,
                            pos.0
                        );
                        if pos.0 == 6 && state.board[4][pos.1] == 0 {
                            //Black double pawn move
                            let m = format!(
                                "{}{}-{}{}",
                                (pos.0 + 97) as u8 as char,
                                pos.1 + 1,
                                (pos.0 + 97) as u8 as char,
                                pos.1 - 1
                            );
                            all_moves.push(m);
                        } else if pos.0 == 1 {
                            for piece in PIECE_SYMBOLS.iter() {
                                all_moves.push(m.clone() + piece);
                            }
                        } else {
                            all_moves.push(m);
                        }
                    }
                    if pos.1 > 0
                        && (state.board[pos.0 - 1][pos.1 - 1] > 0
                            || (state.en_passant[pos.1 - 1] != 0 && pos.0 == 3))
                        && !(vertical_pin.contains(pos)
                            || horisontal_pin.contains(pos)
                            || left_diagonal_pin.contains(pos))
                    {
                        let m = format!(
                            "{}{}x{}{}",
                            (pos.1 + 97) as u8 as char,
                            pos.0 + 1,
                            (pos.1 + 96) as u8 as char,
                            pos.0
                        );
                        if pos.0 == 1 {
                            for piece in PIECE_SYMBOLS.iter() {
                                all_moves.push(m.clone() + piece);
                            }
                        } else {
                            all_moves.push(m.clone());
                        }
                    }
                    if pos.1 < 7
                        && (state.board[pos.0 - 1][pos.1 + 1] > 0
                            || (state.en_passant[pos.1 + 1] != 0 && pos.0 == 3))
                        && !(vertical_pin.contains(pos)
                            || horisontal_pin.contains(pos)
                            || right_diagonal_pin.contains(pos))
                    {
                        let m = format!(
                            "{}{}x{}{}",
                            (pos.1 + 97) as u8 as char,
                            pos.0 + 1,
                            (pos.1 + 98) as u8 as char,
                            pos.0
                        );
                        if pos.0 == 1 {
                            for piece in PIECE_SYMBOLS.iter() {
                                all_moves.push(m.clone() + piece);
                            }
                        } else {
                            all_moves.push(m);
                        }
                    }
                }
                for pos in r_pos.iter() {
                    let mut possible: Vec<(usize, usize)> = Vec::new();
                    if !(vertical_pin.contains(pos)
                        || right_diagonal_pin.contains(pos)
                        || left_diagonal_pin.contains(pos))
                    {
                        //Horisontal rook moves
                        for dir in HORISONTAL {
                            possible.append(&mut ChessState::directional_moves(
                                state.board,
                                *pos,
                                dir,
                                player,
                            ));
                        }
                    }
                    if !(horisontal_pin.contains(pos)
                        || right_diagonal_pin.contains(pos)
                        || left_diagonal_pin.contains(pos))
                    {
                        //Vertical rook moves
                        for dir in VERTICAL {
                            possible.append(&mut ChessState::directional_moves(
                                state.board,
                                *pos,
                                dir,
                                player,
                            ));
                        }
                    }
                    for new_pos in possible.iter() {
                        all_moves.push(if state.board[new_pos.0][new_pos.1] == 0 {
                            format!(
                                "R{}{}-{}{}",
                                (pos.1 + 97) as u8 as char,
                                pos.0 + 1,
                                (new_pos.1 + 97) as u8 as char,
                                new_pos.0 + 1
                            )
                        } else {
                            format!(
                                "R{}{}x{}{}",
                                (pos.1 + 97) as u8 as char,
                                pos.0 + 1,
                                (new_pos.1 + 97) as u8 as char,
                                new_pos.0 + 1
                            )
                        });
                    }
                }
                for pos in b_pos.iter() {
                    let mut possible: Vec<(usize, usize)> = Vec::new();
                    if !(vertical_pin.contains(pos)
                        || horisontal_pin.contains(pos)
                        || left_diagonal_pin.contains(pos))
                    {
                        //Horisontal rook moves
                        for dir in RIGHT_DIAGONAL {
                            possible.append(&mut ChessState::directional_moves(
                                state.board,
                                *pos,
                                dir,
                                player,
                            ));
                        }
                    }
                    if !(vertical_pin.contains(pos)
                        || horisontal_pin.contains(pos)
                        || right_diagonal_pin.contains(pos))
                    {
                        //Vertical rook moves
                        for dir in LEFT_DIAGONAL {
                            possible.append(&mut ChessState::directional_moves(
                                state.board,
                                *pos,
                                dir,
                                player,
                            ));
                        }
                    }
                    for new_pos in possible.iter() {
                        all_moves.push(if state.board[new_pos.0][new_pos.1] == 0 {
                            format!(
                                "B{}{}-{}{}",
                                (pos.1 + 97) as u8 as char,
                                pos.0 + 1,
                                (new_pos.1 + 97) as u8 as char,
                                new_pos.0 + 1
                            )
                        } else {
                            format!(
                                "B{}{}x{}{}",
                                (pos.1 + 97) as u8 as char,
                                pos.0 + 1,
                                (new_pos.1 + 97) as u8 as char,
                                new_pos.0 + 1
                            )
                        });
                    }
                }
                for pos in n_pos.iter() {
                    if !(vertical_pin.contains(pos)
                        || horisontal_pin.contains(pos)
                        || left_diagonal_pin.contains(pos)
                        || right_diagonal_pin.contains(pos))
                    {
                        for dir in KNIGHT_DIRECTIONS {
                            let new_pos = (pos.0 as i8 + dir.0, pos.1 as i8 + dir.1);
                            if -1 < new_pos.0 && new_pos.0 < 8 && -1 < new_pos.1 && new_pos.1 < 7 {
                                if state.board[new_pos.0 as usize][new_pos.1 as usize] < 1 {
                                    all_moves.push(
                                        if state.board[new_pos.0 as usize][new_pos.1 as usize] == 0
                                        {
                                            format!(
                                                "N{}{}-{}{}",
                                                (pos.1 + 97) as u8 as char,
                                                pos.0 + 1,
                                                (new_pos.1 + 97) as u8 as char,
                                                new_pos.0 + 1
                                            )
                                        } else {
                                            format!(
                                                "N{}{}x{}{}",
                                                (pos.1 + 97) as u8 as char,
                                                pos.0 + 1,
                                                (new_pos.1 + 97) as u8 as char,
                                                new_pos.0 + 1
                                            )
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
                if q_pos.0 != 9 {
                    let mut possible: Vec<(usize, usize)> = Vec::new();
                    if !(vertical_pin.contains(&q_pos)
                        || horisontal_pin.contains(&q_pos)
                        || left_diagonal_pin.contains(&q_pos))
                    {
                        for dir in RIGHT_DIAGONAL {
                            possible.append(&mut ChessState::directional_moves(
                                state.board,
                                q_pos,
                                dir,
                                player,
                            ));
                        }
                    }

                    if !(vertical_pin.contains(&q_pos)
                        || horisontal_pin.contains(&q_pos)
                        || right_diagonal_pin.contains(&q_pos))
                    {
                        for dir in LEFT_DIAGONAL {
                            possible.append(&mut ChessState::directional_moves(
                                state.board,
                                q_pos,
                                dir,
                                player,
                            ));
                        }
                    }

                    if !(vertical_pin.contains(&q_pos)
                        || right_diagonal_pin.contains(&q_pos)
                        || left_diagonal_pin.contains(&q_pos))
                    {
                        for dir in HORISONTAL {
                            possible.append(&mut ChessState::directional_moves(
                                state.board,
                                q_pos,
                                dir,
                                player,
                            ));
                        }
                    }

                    if !(right_diagonal_pin.contains(&q_pos)
                        || horisontal_pin.contains(&q_pos)
                        || left_diagonal_pin.contains(&q_pos))
                    {
                        {
                            for dir in VERTICAL {
                                possible.append(&mut ChessState::directional_moves(
                                    state.board,
                                    q_pos,
                                    dir,
                                    player,
                                ))
                            }
                        }
                    }
                    for new_pos in possible.iter() {
                        all_moves.push(if state.board[new_pos.0][new_pos.1] == 0 {
                            format!(
                                "Q{}{}-{}{}",
                                (q_pos.1 + 97) as u8 as char,
                                q_pos.0 + 1,
                                (new_pos.1 + 97) as u8 as char,
                                new_pos.0 + 1
                            )
                        } else {
                            format!(
                                "Q{}{}x{}{}",
                                (q_pos.1 + 97) as u8 as char,
                                q_pos.0 + 1,
                                (new_pos.1 + 97) as u8 as char,
                                new_pos.0 + 1
                            )
                        });
                    }
                }
                if k_pos.0 != 9 {
                    for dir in KING_DIRECTIONS {
                        let new_pos: (i8, i8) = (k_pos.0 as i8 + dir.0, k_pos.1 as i8 + dir.1);
                        if -1 < new_pos.0
                            && new_pos.0 < 8
                            && -1 < new_pos.1
                            && new_pos.1 < 7
                            && !danger_squares.contains(&(new_pos.0 as usize, new_pos.1 as usize))
                        {
                            all_moves.push(
                                if state.board[new_pos.0 as usize][new_pos.1 as usize] == 0 {
                                    format!(
                                        "K{}{}-{}{}",
                                        (k_pos.1 + 97) as u8 as char,
                                        k_pos.0 + 1,
                                        (new_pos.1 + 97) as u8 as char,
                                        new_pos.0 + 1
                                    )
                                } else {
                                    format!(
                                        "K{}{}x{}{}",
                                        (q_pos.1 + 97) as u8 as char,
                                        q_pos.0 + 1,
                                        (new_pos.1 + 97) as u8 as char,
                                        new_pos.0 + 1
                                    )
                                },
                            );
                        }
                    }
                    if state.castling.contains("k")
                        && state.board[7][5] == 0
                        && state.board[7][6] == 0
                        && !danger_squares.contains(&(7, 5))
                        && !danger_squares.contains(&(7, 6))
                    {
                        all_moves.push("O-O".into())
                    }
                    if state.castling.contains("q")
                        && state.board[7][1] == 0
                        && state.board[7][2] == 0
                        && state.board[7][3] == 0
                        && !danger_squares.contains(&(7, 1))
                        && !danger_squares.contains(&(7, 2))
                        && !danger_squares.contains(&(7, 3))
                    {
                        all_moves.push("O-O-O".into())
                    }
                }
            }
        }
        return all_moves;
    }

    fn directional_moves(
        board: [[i8; 8]; 8],
        position: (usize, usize),
        direction: (i8, i8),
        player: bool,
    ) -> Vec<(usize, usize)> {
        let coefficient: i8 = if player { 1 } else { -1 };
        let mut new_pos = (
            position.0 as i8 + direction.0,
            position.1 as i8 + direction.1,
        );
        let mut moves: Vec<(usize, usize)> = Vec::new();
        while -1 < new_pos.0 && new_pos.0 < 8 && -1 < new_pos.1 && new_pos.1 < 8 {
            //print!("directional_moves while loop   ");
            if board[new_pos.0 as usize][new_pos.1 as usize] == 0 {
                moves.push((new_pos.0 as usize, new_pos.1 as usize));
            } else if board[new_pos.0 as usize][new_pos.1 as usize] * coefficient < 0 {
                moves.push((new_pos.0 as usize, new_pos.1 as usize));
                break;
            } else {
                break;
            }
            new_pos = (new_pos.0 as i8 + direction.0, new_pos.1 as i8 + direction.1);
        }
        return moves;
    }

    fn vertical_pin(
        board: [[i8; 8]; 8],
        position: (usize, usize),
        coefficient: i8,
    ) -> Vec<(usize, usize)> {
        let mut pinned = Vec::new();
        for direction in VERTICAL {
            pinned.append(&mut ChessState::directional_pin(
                board,
                position,
                coefficient,
                direction,
                vec![-2, -5],
            ))
        }
        pinned
    }

    fn horisontal_pin(
        board: [[i8; 8]; 8],
        position: (usize, usize),
        coefficient: i8,
    ) -> Vec<(usize, usize)> {
        let mut pinned = Vec::new();
        for direction in HORISONTAL {
            pinned.append(&mut ChessState::directional_pin(
                board,
                position,
                coefficient,
                direction,
                vec![-2, -5],
            ))
        }
        pinned
    }

    fn right_diagonal_pin(
        board: [[i8; 8]; 8],
        position: (usize, usize),
        coefficient: i8,
    ) -> Vec<(usize, usize)> {
        let mut pinned = Vec::new();
        for direction in RIGHT_DIAGONAL {
            pinned.append(&mut ChessState::directional_pin(
                board,
                position,
                coefficient,
                direction,
                vec![-4, -5],
            ))
        }
        pinned
    }

    fn left_diagonal_pin(
        board: [[i8; 8]; 8],
        position: (usize, usize),
        coefficient: i8,
    ) -> Vec<(usize, usize)> {
        let mut pinned = Vec::new();
        for direction in LEFT_DIAGONAL {
            pinned.append(&mut ChessState::directional_pin(
                board,
                position,
                coefficient,
                direction,
                vec![-4, -5],
            ))
        }
        pinned
    }

    fn directional_pin(
        board: [[i8; 8]; 8],
        position: (usize, usize),
        coefficient: i8,
        direction: (i8, i8),
        dangerous: Vec<i8>,
    ) -> Vec<(usize, usize)> {
        let mut visited: Vec<(usize, usize)> = Vec::new();
        let mut new_pos = (
            position.0 as i8 + direction.0,
            position.1 as i8 + direction.1,
        );
        let mut pin_flag = false;
        while -1 < new_pos.0 && new_pos.0 < 8 && -1 < new_pos.1 && new_pos.1 < 8 {
            if board[new_pos.0 as usize][new_pos.1 as usize] * coefficient < 0 {
                if (dangerous
                    .contains(&(board[new_pos.0 as usize][new_pos.1 as usize] * coefficient)))
                    && pin_flag
                {
                    return visited;
                } else {
                    break;
                }
            } else if board[new_pos.0 as usize][new_pos.1 as usize] * coefficient > 0 {
                if pin_flag {
                    break;
                } else {
                    pin_flag = true;
                    visited.push((new_pos.0 as usize, new_pos.1 as usize));
                }
            } else {
            }
            new_pos = (new_pos.0 as i8 + direction.0, new_pos.1 as i8 + direction.1);
        }
        Vec::new()
    }

    fn danger_squares(board: [[i8; 8]; 8], coefficient: i8) -> HashSet<(usize, usize)> {
        let mut danger_squares = HashSet::new();
        for (idx, row) in board.iter().enumerate() {
            for (jdx, field) in row.iter().enumerate() {
                let position = (idx, jdx);
                match field * coefficient {
                    -1 => {}
                    -2 => {
                        for direction in STRAIGHT {
                            for new_pos in ChessState::danger_squares_from_position(
                                board,
                                coefficient,
                                direction,
                                position,
                            )
                            .iter()
                            {
                                danger_squares.insert(*new_pos);
                            }
                        }
                    }
                    -3 => {
                        for direction in KNIGHT_DIRECTIONS {
                            let new_pos = (
                                (position.0 as i8 + direction.0),
                                (position.1 as i8 + direction.1),
                            );
                            if -1 < new_pos.0 && new_pos.0 < 8 && -1 < new_pos.1 && new_pos.1 < 8 {
                                danger_squares.insert((new_pos.0 as usize, new_pos.1 as usize));
                            }
                        }
                    }
                    -4 => {
                        for direction in DIAGONAL {
                            for new_pos in ChessState::danger_squares_from_position(
                                board,
                                coefficient,
                                direction,
                                position,
                            )
                            .iter()
                            {
                                danger_squares.insert(*new_pos);
                            }
                        }
                    }
                    -5 => {
                        for direction in KING_DIRECTIONS {
                            for new_pos in ChessState::danger_squares_from_position(
                                board,
                                coefficient,
                                direction,
                                position,
                            )
                            .iter()
                            {
                                danger_squares.insert(*new_pos);
                            }
                        }
                    }
                    -6 => {
                        for direction in KING_DIRECTIONS {
                            let new_pos = (
                                (position.0 as i8 + direction.0),
                                (position.1 as i8 + direction.1),
                            );
                            if -1 < new_pos.0 && new_pos.0 < 8 && -1 < new_pos.1 && new_pos.1 < 8 {
                                danger_squares.insert((new_pos.0 as usize, new_pos.1 as usize));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        danger_squares
    }

    fn danger_squares_from_position(
        board: [[i8; 8]; 8],
        coefficient: i8,
        direction: (i8, i8),
        position: (usize, usize),
    ) -> HashSet<(usize, usize)> {
        let mut danger_squares = HashSet::new();
        let mut new_pos = (
            position.0 as i8 + direction.0,
            position.1 as i8 + direction.1,
        );
        while -1 < new_pos.0 && new_pos.0 < 8 && -1 < new_pos.1 && new_pos.1 < 8 {
            if board[new_pos.0 as usize][new_pos.1 as usize] * coefficient < 0 {
                danger_squares.insert((new_pos.0 as usize, new_pos.1 as usize));
                break;
            } else if board[new_pos.0 as usize][new_pos.1 as usize] * coefficient > 0 {
                danger_squares.insert((new_pos.0 as usize, new_pos.1 as usize));
                break;
            } else {
                danger_squares.insert((new_pos.0 as usize, new_pos.1 as usize));
            }
            new_pos = (new_pos.0 as i8 + direction.0, new_pos.1 as i8 + direction.1);
        }
        return danger_squares;
    }

    pub fn is_terminal(state: ChessState) -> bool {
        return ChessState::get_all_possible_moves(state).len() != 0;
    }
}
