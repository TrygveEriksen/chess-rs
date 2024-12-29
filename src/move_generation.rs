use std::usize;

use crate::chess::ChessState;

const HORISONTAL: [(i8, i8); 2] = [(1, 0), (-1, 0)];
const VERTICAL: [(i8, i8); 2] = [(0, 1), (0, -1)]; //
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
        let PIECE_SYMBOLS: [String; 4] = [
            "R".to_string(),
            "B".to_string(),
            "N".to_string(),
            "Q".to_string(),
        ];
        let player: bool = state.turn;
        let coefficient: i8 = if player { 1 } else { -1 };

        let mut all_moves: Vec<String> = Vec::new();
        let p_pos: Vec<(usize, usize)> = ChessState::find_positions(state.board, 1 * coefficient);
        let r_pos: Vec<(usize, usize)> = ChessState::find_positions(state.board, 2 * coefficient);
        let n_pos: Vec<(usize, usize)> = ChessState::find_positions(state.board, 3 * coefficient);
        let b_pos: Vec<(usize, usize)> = ChessState::find_positions(state.board, 4 * coefficient);
        let q_pos: (usize, usize) = ChessState::find_position(state.board, 5 * coefficient);
        let k_pos: (usize, usize) = ChessState::find_position(state.board, 6 * coefficient);
        let horisontal_lock: Vec<(usize, usize)> = ChessState::horisontal_lock();
        let vertical_lock: Vec<(usize, usize)> = ChessState::vertical_lock();
        let left_diagonal_lock: Vec<(usize, usize)> = ChessState::left_diagonal_lock();
        let right_diagonal_lock: Vec<(usize, usize)> = ChessState::right_diagonal_lock();

        if player {
            //white move generation
            if ChessState::king_checked(&state, player) {
            } else {
                //Pawns
                for pos in p_pos.iter() {
                    if state.board[pos.0 + 1][pos.1] == 0 {
                        let m = format!(
                            "{}{}-{}{}",
                            (pos.1 + 97) as u8 as char,
                            pos.0 + 1,
                            (pos.1 + 97) as u8 as char,
                            pos.0 + 2
                        );
                        if pos.0 == 6 {
                            for piece in PIECE_SYMBOLS.iter() {
                                all_moves.push(m.clone() + piece.as_str());
                            }
                        } else {
                            all_moves.push(m);
                        }
                        if pos.1 > 0 && state.board[pos.0 + 1][pos.1 - 1] < 0 {
                            let m = format!(
                                "{}{}x{}{}",
                                (pos.1 + 97) as u8 as char,
                                pos.0 + 1,
                                (pos.1 + 96) as u8 as char,
                                pos.0 + 2
                            );
                            if pos.0 == 6 {
                                for piece in PIECE_SYMBOLS.iter() {
                                    all_moves.push(m.clone() + piece.as_str());
                                }
                            } else {
                                all_moves.push(m.clone());
                            }
                        }
                        if pos.1 < 7 && state.board[pos.0 + 1][pos.1 + 1] < 0 {
                            let m = format!(
                                "{}{}x{}{}",
                                (pos.1 + 97) as u8 as char,
                                pos.0 + 1,
                                (pos.1 + 98) as u8 as char,
                                pos.0 + 2
                            );
                            if pos.0 == 6 {
                                for piece in PIECE_SYMBOLS.iter() {
                                    all_moves.push(m.clone() + piece.as_str());
                                }
                            } else {
                                all_moves.push(m);
                            }
                        }
                        if pos.0 == 1 && state.board[3][pos.0] == 0 {
                            //White double pawn move
                            let m = format!(
                                "{}{}-{}{}",
                                (pos.1 + 97) as u8 as char,
                                2,
                                (pos.1 + 97) as u8 as char,
                                4
                            );
                            all_moves.push(m);
                        }
                    }
                }
                for pos in r_pos.iter() {
                    let mut possible: Vec<(usize, usize)> = Vec::new();
                    if !(vertical_lock.contains(pos)
                        || right_diagonal_lock.contains(pos)
                        || left_diagonal_lock.contains(pos))
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
                    if !(horisontal_lock.contains(pos)
                        || right_diagonal_lock.contains(pos)
                        || left_diagonal_lock.contains(pos))
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
                    if !(vertical_lock.contains(pos)
                        || horisontal_lock.contains(pos)
                        || left_diagonal_lock.contains(pos))
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
                    if !(vertical_lock.contains(pos)
                        || horisontal_lock.contains(pos)
                        || right_diagonal_lock.contains(pos))
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
                    if !(vertical_lock.contains(pos)
                        || horisontal_lock.contains(pos)
                        || horisontal_lock.contains(pos)
                        || right_diagonal_lock.contains(pos))
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
                    if !(vertical_lock.contains(&q_pos)
                        || horisontal_lock.contains(&q_pos)
                        || left_diagonal_lock.contains(&q_pos))
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

                    if !(vertical_lock.contains(&q_pos)
                        || horisontal_lock.contains(&q_pos)
                        || right_diagonal_lock.contains(&q_pos))
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

                    if !(vertical_lock.contains(&q_pos)
                        || right_diagonal_lock.contains(&q_pos)
                        || left_diagonal_lock.contains(&q_pos))
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

                    if !(right_diagonal_lock.contains(&q_pos)
                        || horisontal_lock.contains(&q_pos)
                        || left_diagonal_lock.contains(&q_pos))
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
                            && ChessState::safe_square(
                                state.board,
                                player,
                                (new_pos.0 as usize, new_pos.1 as usize),
                            )
                        {
                            all_moves.push(
                                if state.board[new_pos.0 as usize][new_pos.1 as usize] == 0 {
                                    format!(
                                        "K{}{}-{}{}",
                                        (q_pos.1 + 97) as u8 as char,
                                        q_pos.0 + 1,
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
            if ChessState::king_checked(&state, player) {
            } else {
                //Pawns
                for pos in p_pos.iter() {
                    if state.board[pos.0 - 1][pos.1] == 0 {
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
                                all_moves.push(m.clone() + piece.as_str());
                            }
                        } else {
                            all_moves.push(m);
                        }
                    }
                    if pos.1 > 0 && state.board[pos.0 - 1][pos.1 - 1] > 0 {
                        let m = format!(
                            "{}{}x{}{}",
                            (pos.1 + 97) as u8 as char,
                            pos.0 + 1,
                            (pos.1 + 96) as u8 as char,
                            pos.0
                        );
                        if pos.0 == 1 {
                            for piece in PIECE_SYMBOLS.iter() {
                                all_moves.push(m.clone() + piece.as_str());
                            }
                        } else {
                            all_moves.push(m.clone());
                        }
                    }
                    if pos.1 < 7 && state.board[pos.0 - 1][pos.1 + 1] > 0 {
                        let m = format!(
                            "{}{}x{}{}",
                            (pos.1 + 97) as u8 as char,
                            pos.0 + 1,
                            (pos.1 + 98) as u8 as char,
                            pos.0
                        );
                        if pos.0 == 1 {
                            for piece in PIECE_SYMBOLS.iter() {
                                all_moves.push(m.clone() + piece.as_str());
                            }
                        } else {
                            all_moves.push(m);
                        }
                    }
                }
                for pos in r_pos.iter() {
                    let mut possible: Vec<(usize, usize)> = Vec::new();
                    if !(vertical_lock.contains(pos)
                        || right_diagonal_lock.contains(pos)
                        || left_diagonal_lock.contains(pos))
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
                    if !(horisontal_lock.contains(pos)
                        || right_diagonal_lock.contains(pos)
                        || left_diagonal_lock.contains(pos))
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
                    if !(vertical_lock.contains(pos)
                        || horisontal_lock.contains(pos)
                        || left_diagonal_lock.contains(pos))
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
                    if !(vertical_lock.contains(pos)
                        || horisontal_lock.contains(pos)
                        || right_diagonal_lock.contains(pos))
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
                    if !(vertical_lock.contains(pos)
                        || horisontal_lock.contains(pos)
                        || horisontal_lock.contains(pos)
                        || right_diagonal_lock.contains(pos))
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
                    if !(vertical_lock.contains(&q_pos)
                        || horisontal_lock.contains(&q_pos)
                        || left_diagonal_lock.contains(&q_pos))
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

                    if !(vertical_lock.contains(&q_pos)
                        || horisontal_lock.contains(&q_pos)
                        || right_diagonal_lock.contains(&q_pos))
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

                    if !(vertical_lock.contains(&q_pos)
                        || right_diagonal_lock.contains(&q_pos)
                        || left_diagonal_lock.contains(&q_pos))
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

                    if !(right_diagonal_lock.contains(&q_pos)
                        || horisontal_lock.contains(&q_pos)
                        || left_diagonal_lock.contains(&q_pos))
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
                            && ChessState::safe_square(
                                state.board,
                                player,
                                (new_pos.0 as usize, new_pos.1 as usize),
                            )
                        {
                            all_moves.push(
                                if state.board[new_pos.0 as usize][new_pos.1 as usize] == 0 {
                                    format!(
                                        "K{}{}-{}{}",
                                        (q_pos.1 + 97) as u8 as char,
                                        q_pos.0 + 1,
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

    fn vertical_lock() -> Vec<(usize, usize)> {
        Vec::new()
    }

    fn horisontal_lock() -> Vec<(usize, usize)> {
        Vec::new()
    }

    fn right_diagonal_lock() -> Vec<(usize, usize)> {
        Vec::new()
    }

    fn left_diagonal_lock() -> Vec<(usize, usize)> {
        Vec::new()
    }
    fn safe_square(board: [[i8; 8]; 8], player: bool, square: (usize, usize)) -> bool {
        return board[square.0][square.1] == 0;
    }

    pub fn king_checked(state: &ChessState, player: bool) -> bool {
        return false;
    }

    pub fn is_terminal() -> bool {
        return false;
    }
}
