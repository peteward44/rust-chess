use super::boardstate::{BoardPiece, BoardState};
use super::consts;

// Enforces move rule constraints


pub struct Move {
	x: usize,
	y: usize,
}


pub struct Rules {}


impl Rules {
	pub fn can_select_square(
		board_state: &BoardState,
		x: usize,
		y: usize,
	) -> bool {
		return board_state.get_square(x, y) != None;
	}

	pub fn get_piece_possible_moves(
		board_state: &BoardState,
		x: usize,
		y: usize,
		piece: &BoardPiece,
	) -> Vec<Move> {
		let mut vec = vec![];

		match piece.piece {
			consts::PieceType::PAWN => {
				vec.push(Move { x: x, y: y + 1 });
			}
			_ => {}
		}

		vec
	}
}
