use bevy::{prelude::*, render::pass::ClearColor};

mod board;
mod consts;
mod input;
mod network;
mod piecemanager;
mod scalecamera;
mod sprite_picker;

use board::BoardPlugin;
use input::InputPlugin;
use network::NetworkPlugin;
use piecemanager::PieceManagerPlugin;

/// An implementation of the classic game "Breakout"
fn main() {
	let vsync = false;
	let mut builder = App::build();

	builder
		.add_resource(WindowDescriptor {
			title: "Chess".to_string(),
			width: 1366 as u32,
			height: 768 as u32,
			resizable: true,
			// mode: window::WindowMode::Fullscreen {use_size: false},
			mode: bevy::window::WindowMode::Windowed,
			vsync: vsync,
			..Default::default()
		})
		.add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
		.add_plugins(DefaultPlugins)
		.add_plugin(BoardPlugin)
		.add_plugin(InputPlugin)
		.add_plugin(NetworkPlugin)
		.add_plugin(PieceManagerPlugin)
		.add_plugin(scalecamera::ScaleCameraPlugin)
		.add_plugin(sprite_picker::SpritePickerPlugin)
		.add_startup_system(setup.system());

	builder.run();
}

fn setup(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	_asset_server: Res<AssetServer>,
) {
	commands
		.spawn(Camera2dComponents::default())
		.with(scalecamera::ScaleCamera::default())
		.with(sprite_picker::SpritePickerCamera)
		//	.spawn(UiCameraComponents::default())
		// player
		.spawn(SpriteComponents {
			material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
			sprite: Sprite::new(Vec2::new(
				scalecamera::DRAW_WINDOW_W,
				scalecamera::DRAW_WINDOW_H,
			)),
			..Default::default()
		});
	// .with(Player)
	// .with(Position { position: Vec2::new(0.0, 0.0) });
	// map
	// let mut rng = rand::thread_rng();
	// for y in 0..10 {
	// for x in 0..10 {
	// commands
	// .spawn(SpriteComponents {
	// material: materials.add(Color::rgb( rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)).into()),
	// transform: Transform::from_translation(Vec3::new(( x as f32 ) * 20.0, ( y as f32 ) * 20.0, 0.0)),
	// sprite: Sprite::new(Vec2::new(32.0, 32.0)),
	// ..Default::default()
	// })
	// .with(Tile { x: x, y: y });
	// }
	// }
}
