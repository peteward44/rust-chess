use super::consts;
use bevy::prelude::*;
use bevy::asset::LoadState;

pub enum PieceType {
	ROOK,
	KNIGHT,
	BISHOP,
	KING,
	QUEEN,
	PAWN,
}

// classes
pub struct Piece {
	pub name: PieceType,
	pub is_white: bool,
	pub captured: bool,
	pub x: i32,
	pub y: i32,
}

impl Piece {
	pub fn new(x: i32, y: i32, name: PieceType, is_white: bool) -> Piece {
		Piece {
			name: name,
			is_white: is_white,
			captured: false,
			x: x,
			y: y,
		}
	}
}

// entity
pub struct PieceManagerPlugin;

impl Plugin for PieceManagerPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
			.add_system_set(SystemSet::on_enter(consts::GameState::LoadingTextures).with_system(load_textures.system()))
			.add_system_set(SystemSet::on_update(consts::GameState::LoadingTextures).with_system(check_textures.system()))
			.add_system_set(SystemSet::on_exit(consts::GameState::LoadingTextures).with_system(startup.system()));
	}
}


fn load_textures(asset_server: Res<AssetServer>) {
	 let texture_handle: Handle<Texture> = asset_server.load("textures/pieces.png");
}


fn check_textures(
    mut state: ResMut<State<consts::GameState>>,
    asset_server: Res<AssetServer>,
) {
	let handle: Handle<Texture> = asset_server.get_handle( "textures/pieces.png" );
	if let LoadState::Loaded = asset_server.get_load_state( handle ) {
		state.set( consts::GameState::LoadingEverythingElse ).unwrap();
	}
}


fn piecetype_to_sprite_index(piece_type: &PieceType, is_white: bool) -> u32 {
	let mut base = 0;
	if is_white == true {
		base = 6;
	}
	match piece_type {
		PieceType::ROOK => base + 0,
		PieceType::KNIGHT => base + 4,
		PieceType::BISHOP => base + 1,
		PieceType::KING => base + 3,
		PieceType::QUEEN => base + 2,
		PieceType::PAWN => base + 5,
	}
}

fn add_piece(
	mut commands: &mut Commands,
	mut _materials: &mut ResMut<Assets<ColorMaterial>>,
	_asset_server: &Res<AssetServer>,
	texture_atlas_handle: Handle<TextureAtlas>,
	piece: Piece,
) {
	let index = piecetype_to_sprite_index(&piece.name, piece.is_white);
	let pos = consts::get_square_position(piece.x, piece.y);
	commands
		.spawn_bundle(SpriteSheetBundle {
			transform: Transform {
				translation: Vec3::new(pos.0, pos.1, 0.0),
				//scale: Vec3::splat(0.05),
				..Default::default()
			},
			sprite: TextureAtlasSprite::new(index),
			texture_atlas: texture_atlas_handle,
			..Default::default()
		})
		.insert(piece);
}



fn startup(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let texture_handle: Handle<Texture> = asset_server.get_handle( "textures/pieces.png" );
	let texture_atlas = TextureAtlas::from_grid(
		texture_handle,
		Vec2::new(consts::PIECE_WIDTH, consts::PIECE_HEIGHT),
		6,
		2,
	);
	let texture_atlas_handle = texture_atlases.add(texture_atlas);

	for side in 0..2 {
		let is_white = side == 1;
		let mut first_row = 0;
		let mut second_row = 1;
		if is_white {
			first_row = 7;
			second_row = 6;
		}
		add_piece(
			&mut commands,
			&mut materials,
			&asset_server,
			texture_atlas_handle.clone(),
			Piece::new(0, first_row, PieceType::ROOK, is_white),
		);
		add_piece(
			&mut commands,
			&mut materials,
			&asset_server,
			texture_atlas_handle.clone(),
			Piece::new(7, first_row, PieceType::ROOK, is_white),
		);
		add_piece(
			&mut commands,
			&mut materials,
			&asset_server,
			texture_atlas_handle.clone(),
			Piece::new(1, first_row, PieceType::KNIGHT, is_white),
		);
		add_piece(
			&mut commands,
			&mut materials,
			&asset_server,
			texture_atlas_handle.clone(),
			Piece::new(6, first_row, PieceType::KNIGHT, is_white),
		);
		add_piece(
			&mut commands,
			&mut materials,
			&asset_server,
			texture_atlas_handle.clone(),
			Piece::new(2, first_row, PieceType::BISHOP, is_white),
		);
		add_piece(
			&mut commands,
			&mut materials,
			&asset_server,
			texture_atlas_handle.clone(),
			Piece::new(5, first_row, PieceType::BISHOP, is_white),
		);
		add_piece(
			&mut commands,
			&mut materials,
			&asset_server,
			texture_atlas_handle.clone(),
			Piece::new(3, first_row, PieceType::QUEEN, is_white),
		);
		add_piece(
			&mut commands,
			&mut materials,
			&asset_server,
			texture_atlas_handle.clone(),
			Piece::new(4, first_row, PieceType::KING, is_white),
		);

		for x in 0..consts::BOARD_WIDTH {
			add_piece(
				&mut commands,
				&mut materials,
				&asset_server,
				texture_atlas_handle.clone(),
				Piece::new(x, second_row, PieceType::PAWN, is_white),
			);
		}
	}
}

// fn system(
// time: Res<Time>,
// keyboard_input: Res<Input<KeyCode>>,
// client: ResMut<Mutex<websocket::sync::Client<std::net::TcpStream>>>,
// mut query: Query<(&Player, &mut Transform)>,
// ) {

// }
