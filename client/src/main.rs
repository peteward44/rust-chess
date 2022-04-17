use bevy::{prelude::*};

mod board;
mod boardstate;
mod boardstatesync;
mod consts;
mod hitarea;
mod input;
mod loading;
mod menu;
mod network;
mod quit;
mod rules;
mod scalecamera;

use board::BoardPlugin;
use input::InputPlugin;
use network::NetworkPlugin;


fn main() {
	let vsync = false;
	let mut app = App::new();

	app
		.insert_resource(WindowDescriptor {
			title: "Chess".to_string(),
			width: 1366.0,
			height: 768.0,
			resizable: true,
			// mode: window::WindowMode::Fullscreen {use_size: false},
			mode: bevy::window::WindowMode::Windowed,
			vsync: vsync,
			..Default::default()
		})
		.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
		.add_state(consts::GameState::Init)
		.add_plugins(DefaultPlugins)
		.add_plugin(loading::LoadTexturesPlugin)
		.add_plugin(menu::MenuPlugin)
		.add_plugin(BoardPlugin)
		.add_plugin(InputPlugin)
		.add_plugin(NetworkPlugin)
		.add_plugin(boardstatesync::BoardStateSyncPlugin)
		.add_plugin(scalecamera::ScaleCameraPlugin)
		.add_plugin(hitarea::HitAreaPlugin)
		.add_plugin(quit::QuitPlugin)
		.add_startup_system(setup.system());

	app.run();
}

fn setup(
	mut commands: Commands,
	mut state: ResMut<State<consts::GameState>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	commands
		.spawn_bundle(OrthographicCameraBundle::new_2d())
		.insert(scalecamera::ScaleCamera::default())
		.insert(hitarea::HitAreaCamera);
	//	.spawn(UiCameraBundle::default())
	// background
	commands.spawn_bundle(SpriteBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
		sprite: Sprite {
			custom_size: Some(Vec2::new(scalecamera::DRAW_WINDOW_W, scalecamera::DRAW_WINDOW_H)),
			color: Color::rgb(0.5, 0.5, 1.0).into(),
			..Default::default()
		},
		..Default::default()
	});

	state.set(consts::GameState::Loading).unwrap();
}
