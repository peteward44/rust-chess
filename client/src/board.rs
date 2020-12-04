use bevy::{
	prelude::*,
	render::pass::ClearColor,
	sprite::collide_aabb::{collide, Collision},
};


pub struct Board;



impl Board {
	pub fn new() -> Board {
		Board {
		}
	}
}


pub struct PieceMoved;


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


