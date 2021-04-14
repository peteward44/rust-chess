// sizes
pub const BOARD_WIDTH: usize = 8;
pub const BOARD_HEIGHT: usize = 8;

pub const SQUARE_WIDTH: f32 = 180.0;
pub const SQUARE_HEIGHT: f32 = 180.0;

pub const PIECE_WIDTH: f32 = 135.0;
pub const PIECE_HEIGHT: f32 = 180.0;

// cosmetics
pub const BOARD_COLOUR1: (f32, f32, f32) = (0.9, 0.9, 0.6);
pub const BOARD_COLOUR2: (f32, f32, f32) = (1.0, 0.45, 0.0);

pub fn get_square_position(
	x: usize,
	y: usize,
) -> (f32, f32) {
	let start_x = -SQUARE_WIDTH * ((BOARD_WIDTH as f32 / 2.0) - 0.5);
	let start_y = -SQUARE_HEIGHT * ((BOARD_HEIGHT as f32 / 2.0) - 0.5);
	(
		start_x + (x as f32) * SQUARE_WIDTH,
		start_y + (y as f32) * SQUARE_HEIGHT,
	)
}


#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
	Init,
	Loading,
	Menu,
	Playing,
	GameOver,
	Quit,
}


#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum PieceType {
	ROOK,
	KNIGHT,
	BISHOP,
	KING,
	QUEEN,
	PAWN,
}
