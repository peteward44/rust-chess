use crate::boardstate::{BoardPiece, BoardState};
use crate::consts;
use bevy::{prelude::*};

// reflects any changes to the BoardState resource to the piece sprites displayed

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


fn add_piece(
	x: i32,
	y: i32,
	commands: &mut Commands,
	texture_atlas_handle: Handle<TextureAtlas>,
	piece: &BoardPiece,
) {
	let index = piecetype_to_sprite_index(&piece.piece, piece.is_white);
	let pos = consts::get_square_position(x, y);
	commands
		.spawn_bundle(SpriteSheetBundle {
			transform: Transform {
				translation: Vec3::new(pos.0, pos.1, 0.1),
				scale: Vec3::new(consts::PIECE_RENDER_WIDTH / consts::PIECE_TEXTURE_WIDTH, consts::PIECE_RENDER_HEIGHT / consts::PIECE_TEXTURE_HEIGHT, 1.0),
				..Default::default()
			},
			sprite: TextureAtlasSprite::new(index),
			texture_atlas: texture_atlas_handle,
			..Default::default()
		})
		.insert(piece.clone());
}


fn on_enter(
	mut commands: Commands,
	board_state: Res<BoardState>,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let texture_handle: Handle<Image> = asset_server.get_handle("textures/primary/pieces.png");
	let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(consts::PIECE_TEXTURE_WIDTH, consts::PIECE_TEXTURE_HEIGHT), 6, 2);
	let texture_atlas_handle = texture_atlases.add(texture_atlas);

	for y in 0..consts::BOARD_HEIGHT {
		for x in 0..consts::BOARD_WIDTH {
			let square = board_state.get_square(x, y);
			match square {
				Some(piece) => {
					add_piece(x, y, &mut commands, texture_atlas_handle.clone(), &piece);
				}
				None => {}
			}
		}
	}
}


fn on_exit(
	mut commands: Commands,
	mut query: Query<(&BoardPiece, Entity)>,
) {
	for (_piece, entity) in query.iter_mut() {
		commands.entity(entity).despawn_recursive();
	}
}


pub struct BoardStateSyncPlugin;


impl Plugin for BoardStateSyncPlugin {
	fn build(
		&self,
		app: &mut App,
	) {
		app.insert_resource(BoardState::default())
			.add_system_set(SystemSet::on_enter(consts::GameState::Playing).with_system(on_enter))
			.add_system_set(SystemSet::on_exit(consts::GameState::Playing).with_system(on_exit));
	}
}
