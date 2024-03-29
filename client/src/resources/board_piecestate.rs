use bevy::prelude::*;
use shakmaty::{Position};
use std::collections::HashMap;
use crate::consts;
use crate::components;

#[derive(Resource)]
pub struct BoardPieceState {
	entity_map: HashMap<shakmaty::Square, Entity>,
}


fn piecetype_to_sprite_index_shakmaty(
	piece_type: shakmaty::Role,
	is_white: shakmaty::Color,
) -> usize {
	let mut base = 0;
	if is_white == shakmaty::Color::White {
		base = 6;
	}
	match piece_type {
		shakmaty::Role::Rook => base + 0,
		shakmaty::Role::Knight => base + 4,
		shakmaty::Role::Bishop => base + 1,
		shakmaty::Role::King => base + 3,
		shakmaty::Role::Queen => base + 2,
		shakmaty::Role::Pawn => base + 5,
	}
}


impl BoardPieceState {
	pub fn new() -> Self {
		BoardPieceState {
			entity_map: HashMap::new(),
		}
	}

	pub fn spawn_ecs_components_shakmaty(
		&mut self,
		commands: &mut Commands,
		asset_server: Res<AssetServer>,
		mut texture_atlases: ResMut<Assets<TextureAtlas>>,
		chess: Res<components::board::ChessResource>,
	) {
		let texture_handle: Handle<Image> = asset_server.get_handle("textures/primary/pieces.png");
		let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(consts::PIECE_TEXTURE_WIDTH, consts::PIECE_TEXTURE_HEIGHT), 6, 2, None, None);
		let texture_atlas_handle = texture_atlases.add(texture_atlas);

		let board = chess.board();

		for square in shakmaty::Square::ALL {
			let piece_option = board.piece_at(square);
			match piece_option {
				Some(piece) => {
					let sprite_index = piecetype_to_sprite_index_shakmaty(piece.role, piece.color);
					let pos = consts::get_square_position(square);
					let entity_id = commands
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
						.insert(components::pieces::BoardPiece{ piece, square })
						.id();
					self.entity_map.insert(square, entity_id);
				},
				_ => {},
			}
		}
	}

	pub fn move_piece(
		&mut self,
		commands: &mut Commands,
		piece: &shakmaty::Piece,
		from: shakmaty::Square,
		to: shakmaty::Square,
	) {
		match self.entity_map.get(&from) {
			Some(&entity) => {
				self.entity_map.remove(&from);
				self.entity_map.insert(to, entity);
				commands.entity(entity).insert(components::pieces::BoardPiece{ piece: *piece, square: to })
					.insert(components::pieces::MoveTo{ from, to });
			},
			_ => {},
		}
	}
}
