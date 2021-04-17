use super::boardstate::{BoardPiece, BoardState};
use super::consts;

// Enforces move rule constraints


pub struct Move {
	pub x: i32,
	pub y: i32,
	pub capture: bool,
}


pub struct Rules {}


impl Rules {
	pub fn is_occupied(
		board_state: &BoardState,
		x: i32,
		y: i32,
	) -> bool {
		return board_state.get_square(x, y) != None;
	}

	pub fn is_occupied_with_colour(
		board_state: &BoardState,
		x: i32,
		y: i32,
		is_white: bool,
	) -> bool {
		let square = &board_state.get_square(x, y);
		match square {
			Some(square_inside) => {
				return square_inside.is_white == is_white;
			}
			None => {
				return false;
			}
		}
	}


	fn gen_move(
		vec: &mut Vec<Move>,
		board_state: &BoardState,
		dx: i32,
		dy: i32,
		piece: &BoardPiece,
	) -> bool {
		if dx < 0 || dy < 0 || dx >= consts::BOARD_WIDTH || dy >= consts::BOARD_HEIGHT {
			return true;
		}
		if !Rules::is_occupied(board_state, dx, dy) {
			vec.push(Move { x: dx, y: dy, capture: false });
			return false;
		} else if Rules::is_occupied_with_colour(board_state, dx, dy, !piece.is_white) {
			vec.push(Move { x: dx, y: dy, capture: true });
			return true;
		}
		return true;
	}

	fn gen_diagonals(
		mut vec: &mut Vec<Move>,
		board_state: &BoardState,
		x: i32,
		y: i32,
		piece: &BoardPiece,
	) {
		let mut dx = x - 1;
		let mut dy = y - 1;
		while dx >= 0 && dy >= 0 {
			if Rules::gen_move(&mut vec, &board_state, dx, dy, &piece) {
				break;
			}
			if dx == 0 || dy == 0 {
				break;
			}
			dx = dx - 1;
			dy = dy - 1;
		}
		dx = x + 1;
		dy = y - 1;
		while dx < consts::BOARD_WIDTH && dy >= 0 {
			if Rules::gen_move(&mut vec, &board_state, dx, dy, &piece) {
				break;
			}
			if dx == consts::BOARD_WIDTH - 1 || dy == 0 {
				break;
			}
			dx = dx + 1;
			dy = dy - 1;
		}
		dx = x - 1;
		dy = y + 1;
		while dx >= 0 && dy < consts::BOARD_HEIGHT {
			if Rules::gen_move(&mut vec, &board_state, dx, dy, &piece) {
				break;
			}
			if dx == 0 || dy == consts::BOARD_HEIGHT - 1 {
				break;
			}
			dx = dx - 1;
			dy = dy + 1;
		}
		dx = x + 1;
		dy = y + 1;
		while dx < consts::BOARD_WIDTH && dy < consts::BOARD_HEIGHT {
			if Rules::gen_move(&mut vec, &board_state, dx, dy, &piece) {
				break;
			}
			if dx == consts::BOARD_WIDTH - 1 || dy == consts::BOARD_HEIGHT - 1 {
				break;
			}
			dx = dx + 1;
			dy = dy + 1;
		}
	}

	fn gen_straights(
		mut vec: &mut Vec<Move>,
		board_state: &BoardState,
		x: i32,
		y: i32,
		piece: &BoardPiece,
	) {
		for dx in (0..x).rev() {
			if Rules::gen_move(&mut vec, &board_state, dx, y, &piece) {
				break;
			}
		}
		for dx in x..consts::BOARD_WIDTH {
			if Rules::gen_move(&mut vec, &board_state, dx, y, &piece) {
				break;
			}
		}
		for dy in (0..y).rev() {
			if Rules::gen_move(&mut vec, &board_state, x, dy, &piece) {
				break;
			}
		}
		for dy in y..consts::BOARD_HEIGHT {
			if Rules::gen_move(&mut vec, &board_state, x, dy, &piece) {
				break;
			}
		}
	}


	// @todo: No bounds checking
	pub fn get_piece_possible_moves(
		board_state: &BoardState,
		x: i32,
		y: i32,
		piece: &BoardPiece,
	) -> Vec<Move> {
		let mut vec: Vec<Move> = vec![];

		match piece.piece {
			consts::PieceType::PAWN => {
				if piece.is_white {
					if !Rules::is_occupied(board_state, x, y + 1) {
						vec.push(Move {
							x: x,
							y: y + 1,
							capture: false,
						});
					}
					// if pawn is still on the first row of the board, they can move 2 squares forward
					if y == 1 {
						if !Rules::is_occupied(board_state, x, y + 2) {
							vec.push(Move {
								x: x,
								y: y + 2,
								capture: false,
							});
						}
					}
					// if there is an enemy piece in a forward diagonal, they can capture.
					if Rules::is_occupied_with_colour(board_state, x - 1, y + 1, !piece.is_white) {
						vec.push(Move {
							x: x - 1,
							y: y + 1,
							capture: true,
						});
					}
					if Rules::is_occupied_with_colour(board_state, x + 1, y + 1, !piece.is_white) {
						vec.push(Move {
							x: x + 1,
							y: y + 1,
							capture: true,
						});
					}
				} else {
					if !Rules::is_occupied(board_state, x, y - 1) {
						vec.push(Move {
							x: x,
							y: y - 1,
							capture: false,
						});
					}
					if y == 6 {
						if !Rules::is_occupied(board_state, x, y - 2) {
							vec.push(Move {
								x: x,
								y: y - 2,
								capture: false,
							});
						}
					}
					// if there is an enemy piece in a forward diagonal, they can capture.
					if Rules::is_occupied_with_colour(board_state, x - 1, y - 1, !piece.is_white) {
						vec.push(Move {
							x: x - 1,
							y: y - 1,
							capture: true,
						});
					}
					if Rules::is_occupied_with_colour(board_state, x + 1, y - 1, !piece.is_white) {
						vec.push(Move {
							x: x + 1,
							y: y - 1,
							capture: true,
						});
					}
				}
			}
			consts::PieceType::ROOK => {
				Rules::gen_straights(&mut vec, &board_state, x, y, &piece);
			}
			consts::PieceType::BISHOP => {
				Rules::gen_diagonals(&mut vec, &board_state, x, y, &piece);
			}
			consts::PieceType::KNIGHT => {
				Rules::gen_move(&mut vec, &board_state, x + 1, y + 2, &piece);
				Rules::gen_move(&mut vec, &board_state, x - 1, y + 2, &piece);
				Rules::gen_move(&mut vec, &board_state, x + 1, y - 2, &piece);
				Rules::gen_move(&mut vec, &board_state, x - 1, y - 2, &piece);

				Rules::gen_move(&mut vec, &board_state, x + 2, y + 1, &piece);
				Rules::gen_move(&mut vec, &board_state, x - 2, y + 1, &piece);
				Rules::gen_move(&mut vec, &board_state, x + 2, y - 1, &piece);
				Rules::gen_move(&mut vec, &board_state, x - 2, y - 1, &piece);
			}
			consts::PieceType::QUEEN => {
				Rules::gen_straights(&mut vec, &board_state, x, y, &piece);
				Rules::gen_diagonals(&mut vec, &board_state, x, y, &piece);
			}
			consts::PieceType::KING => {
				// @todo the king can capture as long as it's not moving into check
				Rules::gen_move(&mut vec, &board_state, x - 1, y - 1, &piece);
				Rules::gen_move(&mut vec, &board_state, x + 0, y - 1, &piece);
				Rules::gen_move(&mut vec, &board_state, x + 1, y - 1, &piece);
				Rules::gen_move(&mut vec, &board_state, x - 1, y + 0, &piece);
				Rules::gen_move(&mut vec, &board_state, x + 1, y + 0, &piece);
				Rules::gen_move(&mut vec, &board_state, x - 1, y + 1, &piece);
				Rules::gen_move(&mut vec, &board_state, x + 0, y + 1, &piece);
				Rules::gen_move(&mut vec, &board_state, x + 1, y + 1, &piece);
			}
		}

		vec
	}
}
