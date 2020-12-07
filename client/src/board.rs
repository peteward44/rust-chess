use super::consts;
use bevy::{
	prelude::*,
};

// classes
#[allow(dead_code)]
struct Square {
	x: i32,
	y: i32,
}


// events
pub struct PieceMoved;

// entity
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_event::<PieceMoved>()
			.add_startup_system(startup.system());
		//			.add_system(system.system());
	}
}

fn startup(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	_asset_server: Res<AssetServer>,
) {
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
	for y in 0..consts::BOARD_WIDTH {
		for x in 0..consts::BOARD_HEIGHT {
			let mut colour = white;
			if ((y % 2) - (x % 2)) == 0 {
				colour = black;
			}
			let pos = consts::get_square_position(x, y);
			commands
				.spawn(SpriteComponents {
					material: materials.add(colour.into()),
					transform: Transform::from_translation(Vec3::new(pos.0, pos.1, 0.0)),
					sprite: Sprite::new(Vec2::new(consts::SQUARE_WIDTH, consts::SQUARE_HEIGHT)),
					..Default::default()
				})
				.with(Square { x: x, y: y });
		}
	}
}

//fn system(_my_events: Res<Events<PieceMoved>>) {}
// fn system(
// time: Res<Time>,
// keyboard_input: Res<Input<KeyCode>>,
// client: ResMut<Mutex<websocket::sync::Client<std::net::TcpStream>>>,
// mut query: Query<(&Player, &mut Transform)>,
// ) {

// }
