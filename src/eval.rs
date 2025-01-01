use crate::chess_init::ChessState;

impl ChessState {
    pub fn eval(state: ChessState) -> i32 {
        let mut new_state = state.copy();
        new_state.turn = true;
        let a: i32 = new_state.get_all_possible_moves().len() as i32;
        new_state.turn = false;
        let b: i32 = new_state.get_all_possible_moves().len() as i32;
        return a - b;
    }
}
