use bevy::{prelude::*};

mod consts;
mod plugins;
mod systems;
mod components;
mod resources;

fn main() {
	let mut app = App::new();

	app
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				title: "Chess".to_string(),
				width: 1366.0,
				height: 768.0,
				resizable: true,
				// mode: window::WindowMode::Fullscreen {use_size: false},
				mode: bevy::window::WindowMode::Windowed,
				present_mode: bevy::window::PresentMode::Immediate,
				..Default::default()
			},
			..Default::default()
		}))
		.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
		.add_state(consts::GameState::Loading)
		//.add_plugins(DefaultPlugins)
		.add_plugin(plugins::loading::LoadTexturesPlugin)
		.add_plugin(plugins::scalecamera::ScaleCameraPlugin)
		.add_plugin(plugins::hitarea::HitAreaPlugin)
		.add_startup_system(setup)

		// GUI
		.add_system(systems::gui_mainmenu::button_system)
			.add_system_set(SystemSet::on_enter(consts::GameState::Menu).with_system(systems::gui_mainmenu::on_enter))
			.add_system_set(SystemSet::on_exit(consts::GameState::Menu).with_system(systems::gui_mainmenu::on_exit))

		.insert_resource(resources::gui_quit::GuiQuit::new())
			.add_system_set(SystemSet::on_enter(consts::GameState::Quit).with_system(systems::gui_quit::on_enter))
			.add_system_set(SystemSet::on_update(consts::GameState::Quit).with_system(systems::gui_quit::on_update))

		.insert_resource(resources::board_renderstate::BoardRenderState::new())
		.add_event::<components::board::SquareSelectedEvent>()
		.add_startup_system(systems::board::on_startup)
		// 	.add_event::<PieceMoved>()
			.add_system_set(SystemSet::on_enter(consts::GameState::Playing).with_system(systems::board::on_enter))
			.add_system_set(
				SystemSet::on_update(consts::GameState::Playing)
					.with_system(systems::board::square_clicked)
					.with_system(systems::board::escape_key)
					.with_system(systems::board::on_piece_moveto)
					.with_system(systems::board::change_square_colour_on_selected_change)
					.with_system(systems::board::change_square_colour_on_possible_move_change)
					.with_system(systems::board::show_possible_moves_on_state_change)
			)
			.add_system_set(SystemSet::on_exit(consts::GameState::Playing).with_system(systems::board::on_exit))

		.insert_resource(resources::board_piecestate::BoardPieceState::new())
		.insert_resource(components::board::ChessResource::new());

	app.run();
}

fn setup(
	mut commands: Commands,
	mut _state: ResMut<State<consts::GameState>>,
) {
	commands
		.spawn_bundle(Camera2dBundle::default())
		.insert(plugins::scalecamera::ScaleCamera::default())
		.insert(plugins::hitarea::HitAreaCamera);
	// background
	commands.spawn_bundle(SpriteBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
		sprite: Sprite {
			custom_size: Some(Vec2::new(plugins::scalecamera::DRAW_WINDOW_W, plugins::scalecamera::DRAW_WINDOW_H)),
			color: Color::rgb(0.5, 0.5, 1.0).into(),
			..Default::default()
		},
		..Default::default()
	});
}
