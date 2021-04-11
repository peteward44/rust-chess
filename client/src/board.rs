use crate::consts;
use crate::hitarea::{MouseClick, SpritePicker};
use bevy::prelude::*;
use std::collections::HashMap;
//use std::rc::Rc;

// classes
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Square {
	x: i32,
	y: i32,
	material: Handle<ColorMaterial>,
}

// resources
struct BoardState {
	squares: HashMap<String, Square>,
	selected: Option<Square>,
}

// events
pub struct PieceMoved;


fn get_square_color( x: i32, y: i32 ) -> Color {
	let white = Color::rgb(
		consts::BOARD_COLOUR1.0,
		consts::BOARD_COLOUR1.1,
		consts::BOARD_COLOUR1.2,
	);
	let black = Color::rgb(
		consts::BOARD_COLOUR2.0,
		consts::BOARD_COLOUR2.1,
		consts::BOARD_COLOUR2.2,
	);
	if ((y % 2) - (x % 2)) == 0 {
		return black;
	}
	return white;
}

fn on_enter(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	_asset_server: Res<AssetServer>,
	mut board_state: ResMut<BoardState>,
) {
	for y in 0..consts::BOARD_WIDTH {
		for x in 0..consts::BOARD_HEIGHT {
			let colour = get_square_color( x, y );
			let material = materials.add(colour.into());
			let square = Square { x: x, y: y, material: material.clone() };
			let name = format!("{} {}", x, y);
			let pos = consts::get_square_position(x, y);
			commands
				.spawn_bundle(SpriteBundle {
					material: material,
					transform: Transform::from_translation(Vec3::new(pos.0, pos.1, 0.0)),
					sprite: Sprite::new(Vec2::new(consts::SQUARE_WIDTH, consts::SQUARE_HEIGHT)),
					..Default::default()
				})
				.insert(SpritePicker::new(&name));
//				.insert(square.clone());
			board_state.squares.insert( name, square );
		}
	}
}

fn square_clicked(
	mut my_event_reader: EventReader<MouseClick>,
	mut board_state: ResMut<BoardState>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	for my_event in my_event_reader.iter() {
		let square = board_state.squares.get(&my_event.name).unwrap();
		println!("{:?} {:?}", square.x, square.y);
		match &board_state.selected {
			Some(selected_square) => {
				// reset already-selected square to original colour
				let mut color_mat = materials.get_mut(&selected_square.material).unwrap();
				color_mat.color = get_square_color( selected_square.x, selected_square.y );
			},
			None => {},
		}
		// set newly selected square to selected colour
		let mut color_mat = materials.get_mut(&square.material).unwrap();
		color_mat.color = Color::rgb(1.0, 1.0, 1.0);
		board_state.selected = Some(square.clone());
	}
}


fn on_exit(
	mut commands: Commands,
	mut query: Query<(&Square, Entity)>
) {
	for (_square, entity) in query.iter_mut() {
		commands.entity( entity ).despawn_recursive();
	}
}


fn escape_key(
	keys: Res<Input<KeyCode>>,
    mut state: ResMut<State<consts::GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
		state.set( consts::GameState::Menu );
    }
}


// Plugin
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_event::<PieceMoved>()
			.insert_resource(BoardState {
				squares: HashMap::new(),
				selected: None,
			})
			.add_system_set(SystemSet::on_enter(consts::GameState::Playing).with_system(on_enter.system()))
			.add_system_set(SystemSet::on_update(consts::GameState::Playing)
				.with_system(square_clicked.system())
				.with_system(escape_key.system())
			)
			.add_system_set(SystemSet::on_exit(consts::GameState::Playing).with_system(on_exit.system()));
	}
}
