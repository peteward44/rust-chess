use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlasBuilder};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LoadingState {
	Init,
	Loading,
	Finished,
}

// entity
pub struct LoadingQueue {
	state: LoadingState,
}

impl Default for LoadingQueue {
	fn default() -> Self {
		Self {
			state: LoadingState::Init,
		}
	}
}

fn process( asset_server: Res<AssetServer>, mut query: Query<(&mut LoadingQueue)> ) {
	for loadingQueue in query.iter_mut() {
		match ( loadingQueue.state ) {
			LoadingState::Init => {
				
			},
			LoadingState::Loading => {
				
			},
			LoadingState::Finished => {
				
			},
		}
	}
 //   rpg_sprite_handles.handles = asset_server.load_folder("textures/rpg").unwrap();
}

// fn check_textures(
    // mut state: ResMut<State<AppState>>,
    // rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    // asset_server: Res<AssetServer>,
// ) {
    // if let LoadState::Loaded =
        // asset_server.get_group_load_state(rpg_sprite_handles.handles.iter().map(|handle| handle.id))
    // {
        // state.set(AppState::Finished).unwrap();
    // }
// }


pub struct LoadingQueuePlugin;

impl Plugin for LoadingQueuePlugin {
	fn build(&self, app: &mut AppBuilder) {
        app
			.add_system(process.system());
	}
}

