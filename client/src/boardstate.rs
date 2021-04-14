use crate::consts;


#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct BoardPiece {
	pub piece: consts::PieceType,
	pub is_white: bool,
}


pub struct BoardState {
	pub state: [[Option<BoardPiece>; consts::BOARD_WIDTH]; consts::BOARD_HEIGHT],
}


impl BoardState {
	fn new_game_setup() -> Self {
		let mut state = [[None; consts::BOARD_WIDTH]; consts::BOARD_HEIGHT];
		for side in 0..2 {
			let is_white = side == 1;
			let mut first_row = 0;
			let mut second_row = 1;
			if is_white {
				first_row = 7;
				second_row = 6;
			}
			state[0][first_row] = Some(BoardPiece {
				piece: consts::PieceType::ROOK,
				is_white: is_white,
			});
			state[7][first_row] = Some(BoardPiece {
				piece: consts::PieceType::ROOK,
				is_white: is_white,
			});
			state[1][first_row] = Some(BoardPiece {
				piece: consts::PieceType::KNIGHT,
				is_white: is_white,
			});
			state[6][first_row] = Some(BoardPiece {
				piece: consts::PieceType::KNIGHT,
				is_white: is_white,
			});
			state[2][first_row] = Some(BoardPiece {
				piece: consts::PieceType::BISHOP,
				is_white: is_white,
			});
			state[5][first_row] = Some(BoardPiece {
				piece: consts::PieceType::BISHOP,
				is_white: is_white,
			});
			state[3][first_row] = Some(BoardPiece {
				piece: consts::PieceType::QUEEN,
				is_white: is_white,
			});
			state[4][first_row] = Some(BoardPiece {
				piece: consts::PieceType::KING,
				is_white: is_white,
			});

			for x in 0..consts::BOARD_WIDTH {
				state[x][second_row] = Some(BoardPiece {
					piece: consts::PieceType::PAWN,
					is_white: is_white,
				});
			}
		}
		BoardState { state: state }
	}
}


impl Default for BoardState {
	fn default() -> Self {
		BoardState::new_game_setup()
	}
}
