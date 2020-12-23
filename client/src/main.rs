use bevy::{prelude::*, render::pass::ClearColor};

mod board;
mod consts;
mod input;
mod network;
mod piecemanager;
mod scalecamera;
mod hitarea;

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
			width: 1366.0,
			height: 768.0,
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
		.add_plugin(hitarea::HitAreaPlugin)
		.add_startup_system(setup.system());

	builder.run();
}

fn setup(
	commands: &mut Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	//	_asset_server: Res<AssetServer>,
) {
	commands
		.spawn(Camera2dBundle::default())
		.with(scalecamera::ScaleCamera::default())
		.with(hitarea::HitAreaCamera)
		//	.spawn(UiCameraBundle::default())
		// background
		.spawn(SpriteBundle {
			material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
			sprite: Sprite::new(Vec2::new(
				scalecamera::DRAW_WINDOW_W,
				scalecamera::DRAW_WINDOW_H,
			)),
			..Default::default()
		});
}
