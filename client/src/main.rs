extern crate websocket;
use std::sync::{Arc, Mutex};

use bevy::{
	prelude::*,
	render::pass::ClearColor,
	sprite::collide_aabb::{collide, Collision},
};
use rand::Rng;
use websocket::{ClientBuilder, Message};
//use websocket::client::Client;

mod board;

use board::BoardPlugin;

struct Player;

struct Position {
	position: Vec2
}



struct Tile {
	x: i32,
	y: i32
}


impl Tile {
	pub fn new() -> Tile {
		Tile {
			x: 0,
			y: 0
		}
	}
}


/// An implementation of the classic game "Breakout"
fn main() {
	let vsync = false;
	let mut builder = App::build();

	builder.add_resource(WindowDescriptor {
			title: "Game".to_string(),
			width: 800 as u32,
			height: 600 as u32,
			resizable: true,
			// mode: window::WindowMode::Fullscreen {use_size: false},
			mode: bevy::window::WindowMode::Windowed,
			vsync: vsync,
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(BoardPlugin)
		.add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
		.add_startup_system(setup.system())
		.add_startup_system(network_setup.system())
		.add_system(player_movement_system.system());
		// .add_system(paddle_movement_system.system())
		// .add_system(ball_collision_system.system())
		// .add_system(ball_movement_system.system())
		// .add_system(scoreboard_system.system());

	builder.run();
}


fn network_setup(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>
) {
	let mut client = ClientBuilder::new("ws://127.0.0.1:8080")
		.unwrap()
		.connect_insecure()
		.unwrap();
	let res = Mutex::new( client );
	commands.insert_resource( res );
}


fn setup(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>
) {
	commands
	   // cameras
		.spawn(Camera2dComponents::default())
		.spawn(UiCameraComponents::default())
		// player
		.spawn(SpriteComponents {
			material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
			sprite: Sprite::new(Vec2::new(20.0, 20.0)),
			..Default::default()
		})
		.with(Player)
		.with(Position { position: Vec2::new(0.0, 0.0) });
	// map
	let mut rng = rand::thread_rng();
	for y in 0..10 {
		for x in 0..10 {
			commands
				.spawn(SpriteComponents {
					material: materials.add(Color::rgb( rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)).into()),
					transform: Transform::from_translation(Vec3::new(( x as f32 ) * 20.0, ( y as f32 ) * 20.0, 0.0)),
					sprite: Sprite::new(Vec2::new(20.0, 20.0)),
					..Default::default()
				})
				.with(Tile { x: x, y: y });
		}
	}
}



fn player_movement_system(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	client: ResMut<Mutex<websocket::sync::Client<std::net::TcpStream>>>,
	mut query: Query<(&Player, &mut Transform)>,
) {
	for (player, mut transform) in query.iter_mut() {
		let mut directionx = 0.0;
		let mut directiony = 0.0;
		let mut command = "";

		if keyboard_input.pressed(KeyCode::Left) {
			directionx -= 1.0;
			command = "left";
		}
		if keyboard_input.pressed(KeyCode::Right) {
			directionx += 1.0;
			command = "right";
		}
		if keyboard_input.pressed(KeyCode::Down) {
			directiony -= 1.0;
			command = "down";
		}
		if keyboard_input.pressed(KeyCode::Up) {
			directiony += 1.0;
			command = "up";
		}

		if ( command.len() > 0 ) {
			let translation = &mut transform.translation;

			*translation.x_mut() += time.delta_seconds * directionx * 500.0;
			*translation.x_mut() = translation.x().min(380.0).max(-380.0);

			*translation.y_mut() += time.delta_seconds * directiony * 500.0;
			*translation.y_mut() = translation.y().min(380.0).max(-380.0);

			let message = Message::text(command);
			client.lock().unwrap().send_message(&message).unwrap(); // Send message
		}
	}
}
