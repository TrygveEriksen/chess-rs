use crate::chess_init::ChessState;
pub struct MinimaxBot {
    pub player: bool,
    pub max_depth: u8,
}

pub trait ChessBot {
    fn respond(&self, state: ChessState) -> String;
}

impl ChessBot for MinimaxBot {
    fn respond(&self, state: ChessState) -> String {
        let res = self.minimax_search(state);
        return res.0;
    }
}
impl MinimaxBot {
    fn max_value(&self, state: ChessState, depth: u8, alpha: i32, beta: i32) -> (String, i32) {
        if depth == 0 || state.is_terminal() {
            return (String::new(), ChessState::eval(state));
        }
        let mut alpha_clone = alpha.clone();
        let beta_clone = beta.clone();
        let mut best: (String, i32) = (String::new(), -2147483648);
        for m in state.get_all_possible_moves().iter() {
            let res = self.min_value(state.do_move(m), depth - 1, alpha_clone, beta_clone);
            if res.1 > best.1 {
                best = (m.clone(), res.1);
                alpha_clone = alpha_clone.max(best.1);
            }
            if alpha_clone >= beta_clone {
                return best;
            }
        }
        return best;
    }

    fn min_value(&self, state: ChessState, depth: u8, alpha: i32, beta: i32) -> (String, i32) {
        if depth == 0 || state.is_terminal() {
            return (String::new(), ChessState::eval(state));
        }
        let alpha_clone = alpha.clone();
        let mut beta_clone = beta.clone();
        let mut best: (String, i32) = (String::new(), 2147483647);
        for m in state.get_all_possible_moves().iter() {
            let res = self.max_value(state.do_move(m), depth - 1, alpha_clone, beta_clone);
            if res.1 < best.1 {
                best = (m.clone(), res.1);
                beta_clone = beta_clone.min(best.1);
            }
            if beta_clone <= alpha_clone {
                return best;
            }
        }
        return best;
    }

    pub fn minimax_search(&self, state: ChessState) -> (String, i32) {
        if state.turn {
            self.max_value(state, self.max_depth, -2147483648, 2147483647)
        } else {
            self.min_value(state, self.max_depth, -2147483648, 2147483647)
        }
    }
}
