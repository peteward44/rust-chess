
// sizes
pub const BOARD_WIDTH: i32 = 8;
pub const BOARD_HEIGHT: i32 = 8;

pub const SQUARE_WIDTH: f32 = 180.0;
pub const SQUARE_HEIGHT: f32 = 180.0;

pub const PIECE_TEXTURE_WIDTH: f32 = 300.0;
pub const PIECE_TEXTURE_HEIGHT: f32 = 400.0;

pub const PIECE_RENDER_WIDTH: f32 = 135.0;
pub const PIECE_RENDER_HEIGHT: f32 = 180.0;

// cosmetics
pub const BOARD_COLOUR1: (f32, f32, f32) = (0.9, 0.9, 0.6);
pub const BOARD_COLOUR2: (f32, f32, f32) = (1.0, 0.45, 0.0);

pub fn get_square_position(
	square: shakmaty::Square,
) -> (f32, f32) {
	let start_x = -SQUARE_WIDTH * ((BOARD_WIDTH as f32 / 2.0) - 0.5);
	let start_y = -SQUARE_HEIGHT * ((BOARD_HEIGHT as f32 / 2.0) - 0.5);
	(start_x + (square.file() as i32 as f32) * SQUARE_WIDTH, start_y + (square.rank() as i32 as f32) * SQUARE_HEIGHT)
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GameState {
	Loading,
	Menu,
	Playing,
	GameOver,
	Quit,
}
