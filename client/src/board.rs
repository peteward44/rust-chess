use bevy::{
	prelude::*,
	render::pass::ClearColor,
	sprite::collide_aabb::{collide, Collision},
};

// classes
struct Square {
	x: i32,
	y: i32,
}


pub struct Board;

impl Board {
	pub fn new() -> Board {
		Board {
		}
	}
}


// events
pub struct PieceMoved;


// entity
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
		app
			.add_event::<PieceMoved>()
			.add_startup_system(startup.system());
//			.add_system(system.system());
    }
}


fn startup(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>,
) {
	let square_width = 40.0;
	let square_height = 40.0;
	let white = Color::rgb(0.9, 0.9, 0.6);
	let black = Color::rgb(1.0, 0.45, 0.0);
	for y in 0..8 {
		for x in 0..8 {
			let mut colour = white;
			if ( ( y % 2 ) - ( x % 2 ) ) == 0 {
				colour = black;
			}				
			commands.spawn(SpriteComponents {
				material: materials.add(colour.into()),
				transform: Transform::from_translation(Vec3::new(( x as f32 ) * square_width, ( y as f32 ) * square_height, 0.0)),
				sprite: Sprite::new(Vec2::new(square_width, square_height)),
				..Default::default()
			})
			.with( Square { x: x, y: y } );
		}
	}
}

fn system(
	mut my_events: ResMut<Events<PieceMoved>>,
) {
	
}
// fn system(
	// time: Res<Time>,
	// keyboard_input: Res<Input<KeyCode>>,
	// client: ResMut<Mutex<websocket::sync::Client<std::net::TcpStream>>>,
	// mut query: Query<(&Player, &mut Transform)>,
// ) {
	
// }


