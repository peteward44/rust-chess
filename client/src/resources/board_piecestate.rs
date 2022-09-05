use bevy::prelude::*;
use crate::consts;
use crate::components;


pub struct BoardPieceState {
	state: [[Option<components::pieces::BoardPiece>; consts::BOARD_WIDTH as usize]; consts::BOARD_HEIGHT as usize],
}

fn piecetype_to_sprite_index(
	piece_type: &consts::PieceType,
	is_white: bool,
) -> usize {
	let mut base = 0;
	if is_white == true {
		base = 6;
	}
	match piece_type {
		consts::PieceType::ROOK => base + 0,
		consts::PieceType::KNIGHT => base + 4,
		consts::PieceType::BISHOP => base + 1,
		consts::PieceType::KING => base + 3,
		consts::PieceType::QUEEN => base + 2,
		consts::PieceType::PAWN => base + 5,
	}
}

impl BoardPieceState {
	pub fn new() -> Self {
		let state: [[Option<components::pieces::BoardPiece>; consts::BOARD_WIDTH as usize]; consts::BOARD_HEIGHT as usize] = array_init::array_init(|_x: usize| {
			array_init::array_init(|_y: usize| None)
		});

		BoardPieceState {
			state,
		}
	}

	fn create_piece_component(
		&mut self,
		x: i32,
		y: i32,
		piece_type: consts::PieceType,
		is_white: bool,
		next_id: &mut consts::PieceId,
	) {
		let piece = components::pieces::BoardPiece {
			piece: piece_type,
			is_white,
			id: *next_id,
		};
		*next_id += 1;
		self.state[x as usize][y as usize] = Some(piece);
	}

	pub fn new_game_setup(&mut self, white_playing: bool) {
		let mut next_id: consts::PieceId = 1;
		for side in 0..2 {
			let is_white;
			if white_playing {
				is_white = side == 0;
			} else {
				is_white = side == 1;
			}
			let mut first_row = 0;
			let mut second_row = 1;
			if side == 1 {
				first_row = 7;
				second_row = 6;
			}
			self.create_piece_component(0, first_row, consts::PieceType::ROOK, is_white, &mut next_id);
			self.create_piece_component(7, first_row, consts::PieceType::ROOK, is_white, &mut next_id);
			self.create_piece_component(1, first_row, consts::PieceType::KNIGHT, is_white, &mut next_id);
			self.create_piece_component(6, first_row, consts::PieceType::KNIGHT, is_white, &mut next_id);
			self.create_piece_component(2, first_row, consts::PieceType::BISHOP, is_white, &mut next_id);
			self.create_piece_component(5, first_row, consts::PieceType::BISHOP, is_white, &mut next_id);
			self.create_piece_component(3, first_row, consts::PieceType::QUEEN, is_white, &mut next_id);
			self.create_piece_component(4, first_row, consts::PieceType::KING, is_white, &mut next_id);

			for x in 0..consts::BOARD_WIDTH {
				self.create_piece_component(x, second_row, consts::PieceType::PAWN, is_white, &mut next_id);
			}
		}
	}

	pub fn spawn_ecs_components(
		&self,
		commands: &mut Commands,
		asset_server: Res<AssetServer>,
		mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	) {
		let texture_handle: Handle<Image> = asset_server.get_handle("textures/primary/pieces.png");
		let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(consts::PIECE_TEXTURE_WIDTH, consts::PIECE_TEXTURE_HEIGHT), 6, 2);
		let texture_atlas_handle = texture_atlases.add(texture_atlas);
		for y in 0..consts::BOARD_WIDTH {
			for x in 0..consts::BOARD_HEIGHT {
				match self.state[x as usize][y as usize] {
					Some(piece) => {
						let sprite_index = piecetype_to_sprite_index(&piece.piece, piece.is_white);
						let pos = consts::get_square_position(x, y);
						commands
							.spawn_bundle(SpriteSheetBundle {
								transform: Transform {
									translation: Vec3::new(pos.0, pos.1, 0.1),
									scale: Vec3::new(consts::PIECE_RENDER_WIDTH / consts::PIECE_TEXTURE_WIDTH, consts::PIECE_RENDER_HEIGHT / consts::PIECE_TEXTURE_HEIGHT, 1.0),
									..Default::default()
								},
								sprite: TextureAtlasSprite::new(sprite_index),
								texture_atlas: texture_atlas_handle.clone(),
								..Default::default()
							})
							.insert(piece.clone());
					},
					_ => {},
				}
			}
		}
	}

	pub fn get_piece(
		&self,
		x: i32,
		y: i32,
	) -> Option<components::pieces::BoardPiece> {
		self.state[x as usize][y as usize]
	}

	pub fn get_possible_moves(
		&self,
		x: i32,
		y: i32,
	) {
		let piece = self.get_piece(x, y);
		match piece {
			Some(board_piece) => {
				
			},
			_ => {},
		}
	}
}
