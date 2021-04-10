use super::{consts, scalecamera};
use bevy::{asset::LoadState, prelude::*};

// Displays user menu


fn on_enter(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>,
) {
	// black background on top of everything else until it's loaded
	// commands.spawn_bundle(SpriteBundle {
		// material: materials.add(Color::rgb(0.1, 0.1, 0.2).into()),
		// transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
		// sprite: Sprite::new(Vec2::new(
			// scalecamera::DRAW_WINDOW_W,
			// scalecamera::DRAW_WINDOW_H,
		// )),
		// ..Default::default()
	// })
	// .insert( Background );
}


fn on_exit(
	mut commands: Commands,
) {
	// for (background, mut entity) in query.iter_mut() {
		// // despawn loading screen
		// commands.entity( entity ).despawn_recursive();
	// }
}



// Plugin
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
			// .init_resource::<LoadHandles>()
			.add_system_set(SystemSet::on_enter(consts::GameState::Menu).with_system(on_enter.system()))
		//	.add_system_set(SystemSet::on_update(consts::GameState::Menu).with_system(on_update.system()))
			.add_system_set(SystemSet::on_exit(consts::GameState::Menu).with_system(on_exit.system()));
	}
}

