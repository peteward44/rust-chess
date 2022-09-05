use crate::{consts};
use super::scalecamera;
use bevy::{asset::LoadState, prelude::*};

// Responsible for loading all textures in the "primary" folder and displaying a splash screen whilst it's done.
// Starts operating when game state move to "Loading", will move the state to "Menu" when done

#[derive(Component)]
struct Background;


#[derive(Default)]
struct LoadHandles {
	handles: Vec<HandleUntyped>,
}


fn on_enter(
	mut commands: Commands,
	mut load_handles: ResMut<LoadHandles>,
	asset_server: Res<AssetServer>,
) {
	// black background on top of everything else until it's loaded
	commands
		.spawn_bundle(SpriteBundle {
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
			sprite: Sprite {
				custom_size: Some(Vec2::new(scalecamera::DRAW_WINDOW_W, scalecamera::DRAW_WINDOW_H)),
				color: Color::rgb(0.1, 0.1, 0.2).into(),
				..Default::default()
			},
			..Default::default()
		})
		.insert(Background);

	// load textures
	load_handles.handles = asset_server.load_folder("textures/primary").unwrap();
	load_handles.handles.extend(asset_server.load_folder("fonts/primary").unwrap());
}


fn on_update(
	load_handles: ResMut<LoadHandles>,
	mut state: ResMut<State<consts::GameState>>,
	asset_server: Res<AssetServer>,
) {
	if let LoadState::Loaded = asset_server.get_group_load_state(load_handles.handles.iter().map(|handle| handle.id)) {
		state.set(consts::GameState::Menu).unwrap();
	}
}


fn on_exit(
	mut commands: Commands,
	mut query: Query<(&Background, Entity)>,
) {
	for (_background, entity) in query.iter_mut() {
		commands.entity(entity).despawn_recursive();
	}
}


// Plugin
pub struct LoadTexturesPlugin;

impl Plugin for LoadTexturesPlugin {
	fn build(
		&self,
		app: &mut App,
	) {
		app.init_resource::<LoadHandles>()
			.add_system_set(SystemSet::on_enter(consts::GameState::Loading).with_system(on_enter))
			.add_system_set(SystemSet::on_update(consts::GameState::Loading).with_system(on_update))
			.add_system_set(SystemSet::on_exit(consts::GameState::Loading).with_system(on_exit));
	}
}
