use crate::chess_init::ChessState;
use std::collections::HashSet;
impl ChessState {
    fn mobility(&self, player: bool) -> i32 {
        let mut score: i32 = 0;
        let coefficient: i8 = if player { 1 } else { -1 };
        let back_rank: usize = if player { 0 } else { 7 };
        let p_pos: Vec<(usize, usize)> = ChessState::find_positions(self.board, 1 * coefficient);
        let r_pos: Vec<(usize, usize)> = ChessState::find_positions(self.board, 2 * coefficient);
        let n_pos: Vec<(usize, usize)> = ChessState::find_positions(self.board, 3 * coefficient);
        let b_pos: Vec<(usize, usize)> = ChessState::find_positions(self.board, 4 * coefficient);
        let q_pos: Vec<(usize, usize)> = ChessState::find_positions(self.board, 5 * coefficient);
        let k_pos: (usize, usize) = ChessState::find_position(self.board, 6 * coefficient);
        let horisontal_pin: Vec<(usize, usize)> =
            ChessState::horisontal_pin(self.board, k_pos, coefficient);
        let vertical_pin: Vec<(usize, usize)> =
            ChessState::vertical_pin(self.board, k_pos, coefficient);
        let left_diagonal_pin: Vec<(usize, usize)> =
            ChessState::left_diagonal_pin(self.board, k_pos, coefficient);
        let right_diagonal_pin: Vec<(usize, usize)> =
            ChessState::right_diagonal_pin(self.board, k_pos, coefficient);
        let danger_squares: HashSet<(usize, usize)> =
            ChessState::danger_squares(self.board, coefficient);

        if player {
            score += ChessState::white_pawn_moves(
                self,
                coefficient,
                &vertical_pin,
                &horisontal_pin,
                &left_diagonal_pin,
                &right_diagonal_pin,
                &p_pos,
            )
            .len() as i32;
        } else {
            score += ChessState::black_pawn_moves(
                self,
                coefficient,
                &vertical_pin,
                &horisontal_pin,
                &left_diagonal_pin,
                &right_diagonal_pin,
                &p_pos,
            )
            .len() as i32;
        }
        score += 5 * ChessState::straight_moves(
            self,
            coefficient,
            &vertical_pin,
            &horisontal_pin,
            &left_diagonal_pin,
            &right_diagonal_pin,
            &r_pos,
            player,
            "R",
        )
        .len() as i32;
        score += 3 * ChessState::diagonal_moves(
            self,
            coefficient,
            &vertical_pin,
            &horisontal_pin,
            &left_diagonal_pin,
            &right_diagonal_pin,
            &b_pos,
            player,
            "B",
        )
        .len() as i32;

        score += 100
            * ChessState::straight_moves(
                self,
                coefficient,
                &vertical_pin,
                &horisontal_pin,
                &left_diagonal_pin,
                &right_diagonal_pin,
                &q_pos,
                player,
                "Q",
            )
            .len() as i32;
        score += 100
            * ChessState::diagonal_moves(
                self,
                coefficient,
                &vertical_pin,
                &horisontal_pin,
                &left_diagonal_pin,
                &right_diagonal_pin,
                &q_pos,
                player,
                "Q",
            )
            .len() as i32;
        score += 3 * ChessState::knight_moves(
            self,
            coefficient,
            &vertical_pin,
            &horisontal_pin,
            &left_diagonal_pin,
            &right_diagonal_pin,
            &n_pos,
        )
        .len() as i32;
        score -= ChessState::king_moves(self, coefficient, &danger_squares, k_pos, back_rank).len()
            as i32;
        return score * coefficient as i32;
    }

    pub fn eval(state: ChessState) -> i32 {
        //let mut evaluation: i32 = 0;
        if state.is_terminal() && state.check() {
            return if state.turn { i32::MIN } else { i32::MAX };
        }
        let mut score: i32 = state.mobility(true) + state.mobility(false);

        return score;
    }
}
