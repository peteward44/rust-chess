use bevy::{
	prelude::*,
	render::pass::ClearColor,
	sprite::collide_aabb::{collide, Collision},
};

pub enum PIECE_TYPE {
	ROOK,
	KNIGHT,
	BISHOP,
	KING,
	QUEEN,
	PAWN,
}

// classes
pub struct Piece {
	pub name: PIECE_TYPE,
	pub is_white: bool,
	pub captured: bool,
	pub x: i32,
	pub y: i32,
}

impl Piece {
	pub fn new( x: i32, y: i32, name: PIECE_TYPE, is_white: bool ) -> Piece {
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
			.add_startup_system(startup.system());
//			.add_system(system.system());
    }
}

fn piece_type_to_sprite_index( piece_type: &PIECE_TYPE, is_white: bool ) -> u32 {
	let mut base = 0;
	if ( is_white == true ) {
		base = 6;
	}
	match piece_type {
		PIECE_TYPE::ROOK => base + 0,
		PIECE_TYPE::KNIGHT => base + 4,
		PIECE_TYPE::BISHOP => base + 1,
		PIECE_TYPE::KING => base + 3,
		PIECE_TYPE::QUEEN => base + 2,
		PIECE_TYPE::PAWN => base + 5,
	}
}

fn add_piece(
	mut commands: &mut Commands,
	mut materials: &mut ResMut<Assets<ColorMaterial>>,
	asset_server: &Res<AssetServer>,
	texture_atlas_handle: Handle<TextureAtlas>,
	piece: Piece,
) {
	let white = Color::rgb(0.9, 0.9, 0.9);
	let black = Color::rgb(0.1, 0.1, 0.1);
	let mut colour = black;
	if ( piece.is_white == true ) {
		colour = white;
	}
	let index = piece_type_to_sprite_index( &piece.name, piece.is_white );
	commands.spawn(SpriteSheetComponents {
		//	material: materials.add(texture_handle.into()),
			transform: Transform {
				translation: Vec3::new(( piece.x as f32 ) * 40.0, ( piece.y as f32 ) * 40.0, 0.0),
				//scale: Vec3::splat(0.05),
				..Default::default()
			},
		//	sprite: Sprite::new(Vec2::new(38.0, 38.0)),
			sprite: TextureAtlasSprite::new(index),
			texture_atlas: texture_atlas_handle,
			..Default::default()
		})
		.with( piece );
}


fn startup(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let texture_handle = asset_server.load("textures/pieces.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(30.0, 40.0), 6, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
	
	for side in 0..2 {
		let is_white = side == 1;
		let mut first_row = 0;
		let mut second_row = 1;
		if ( is_white ) {
			first_row = 7;
			second_row = 6;
		}
		add_piece( &mut commands, &mut materials, &asset_server, texture_atlas_handle.clone(), Piece::new( 0, first_row, PIECE_TYPE::ROOK, is_white ) );
		add_piece( &mut commands, &mut materials, &asset_server, texture_atlas_handle.clone(), Piece::new( 7, first_row, PIECE_TYPE::ROOK, is_white ) );
		add_piece( &mut commands, &mut materials, &asset_server, texture_atlas_handle.clone(), Piece::new( 1, first_row, PIECE_TYPE::KNIGHT, is_white ) );
		add_piece( &mut commands, &mut materials, &asset_server, texture_atlas_handle.clone(), Piece::new( 6, first_row, PIECE_TYPE::KNIGHT, is_white ) );
		add_piece( &mut commands, &mut materials, &asset_server, texture_atlas_handle.clone(), Piece::new( 2, first_row, PIECE_TYPE::BISHOP, is_white ) );
		add_piece( &mut commands, &mut materials, &asset_server, texture_atlas_handle.clone(), Piece::new( 5, first_row, PIECE_TYPE::BISHOP, is_white ) );
		add_piece( &mut commands, &mut materials, &asset_server, texture_atlas_handle.clone(), Piece::new( 3, first_row, PIECE_TYPE::QUEEN, is_white ) );
		add_piece( &mut commands, &mut materials, &asset_server, texture_atlas_handle.clone(), Piece::new( 4, first_row, PIECE_TYPE::KING, is_white ) );

		for x in 0..8 {
			add_piece( &mut commands, &mut materials, &asset_server, texture_atlas_handle.clone(), Piece::new( x, second_row, PIECE_TYPE::PAWN, is_white ) );
		}
	}
}

fn reset() {
	
}

// fn system(
	// time: Res<Time>,
	// keyboard_input: Res<Input<KeyCode>>,
	// client: ResMut<Mutex<websocket::sync::Client<std::net::TcpStream>>>,
	// mut query: Query<(&Player, &mut Transform)>,
// ) {
	
// }


