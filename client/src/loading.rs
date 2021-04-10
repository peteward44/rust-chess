use super::consts;
use bevy::{asset::LoadState, prelude::*};


#[derive(Default)]
struct LoadHandles {
    handles: Vec<HandleUntyped>,
}


fn load_textures(
	mut load_handles: ResMut<LoadHandles>,
	asset_server: Res<AssetServer>,
) {
	load_handles.handles = asset_server.load_folder("textures/primary").unwrap();
}


fn check_textures(
	load_handles: ResMut<LoadHandles>,
    mut state: ResMut<State<consts::GameState>>,
    asset_server: Res<AssetServer>,
) {
	//let handle: Handle<Texture> = asset_server.get_handle( "textures/pieces.png" );
	if let LoadState::Loaded = asset_server.get_group_load_state( load_handles.handles.iter().map( |handle| handle.id ) ) {
		state.set( consts::GameState::Menu ).unwrap();
	}
}



// Plugin
pub struct LoadTexturesPlugin;

impl Plugin for LoadTexturesPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
			.init_resource::<LoadHandles>()
			.add_system_set(SystemSet::on_enter(consts::GameState::Loading).with_system(load_textures.system()))
			.add_system_set(SystemSet::on_update(consts::GameState::Loading).with_system(check_textures.system()));
	}
}

