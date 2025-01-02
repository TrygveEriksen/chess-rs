use crate::chess_init::ChessState;
const PIECES: &str = " PRNBQK";

fn slice_to_coord(col: char, row: char) -> (usize, usize) {
    (
        row.to_digit(10).unwrap() as u8 as usize - 1,
        (col as u8 - 97) as usize,
    )
}
pub fn lan_to_uci(state: &ChessState, m: &String) -> String {
    //Castling
    if m == "O-O-O" {
        return if state.turn {
            "e1a1".to_string()
        } else {
            "e8a8".to_string()
        };
    } else if m == "O-O" {
        return if state.turn {
            "e1g1".to_string()
        } else {
            "e8g8".to_string()
        };
    }
    //Normal pieces
    else if PIECES.contains(m.chars().nth(0).unwrap_or('_')) {
        return m
            .chars()
            .filter(|x| *x != 'x' && *x != '-' && !PIECES.contains(*x))
            .collect::<String>();
    }
    //Pawns
    //Promotion
    else if m.len() == 6 {
        return m
            .chars()
            .filter(|x| *x != 'x' && *x != '-')
            .map(|x| x.to_ascii_lowercase())
            .collect::<String>();
    }
    //normal
    else {
        return m.chars().filter(|x| *x != 'x' && *x != '-').collect();
    }
}

pub fn uci_to_lan(state: &ChessState, m: &String) -> String {
    //Castling
    if m == "e1a1" || m == "e8a8" {
        return "O-O-O".to_string();
    } else if m == "e1g1" || m == "e8g8" {
        return "O-O".to_string();
    }
    //Promotion
    let source = slice_to_coord(m.chars().nth(0).unwrap(), m.chars().nth(1).unwrap());
    let target = slice_to_coord(m.chars().nth(2).unwrap(), m.chars().nth(3).unwrap());
    if m.len() == 5 {
        return m[..2].to_string()
            + if state.board[target.0][target.1] == 0 {
                &"-"
            } else {
                &"x"
            }
            + &m[2..4]
            + &m.chars()
                .nth(4)
                .map(|x| x.to_ascii_uppercase().to_string())
                .unwrap_or_default();
    } else {
        let piece = PIECES
            .chars()
            .nth(state.board[source.0][source.1].abs() as usize)
            .unwrap();
        return piece.to_string()
            + &m[..2]
            + if state.board[target.0][target.1] == 0 {
                &"-"
            } else {
                &"x"
            }
            + &m[2..4];
    }
}
