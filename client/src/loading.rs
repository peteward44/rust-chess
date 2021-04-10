use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlasBuilder};
use std::collections::{ HashMap };
use std::vec::Vec;


#[derive(Debug)]
pub struct LoadingFinished;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LoadingState {
	Init,
	Loading,
	Finished,
}

// entity
pub struct LoadingQueue {
	state: LoadingState,
	handles: HashMap<String, Handle<Texture>>,
	files_to_load: Vec<String>,
}

impl Default for LoadingQueue {
	fn default() -> Self {
		Self {
			state: LoadingState::Init,
			handles: HashMap::new(),
			files_to_load: Vec::new(),
		}
	}
}

impl LoadingQueue {
	pub fn new() -> LoadingQueue {
		LoadingQueue {
			state: LoadingState::Init,
			handles: HashMap::new(),
			files_to_load: Vec::new(),
		}
	}

	fn init_loading( &mut self, asset_server: &Res<AssetServer>, ) {
		for file in self.files_to_load.iter() {
			self.handles.insert( file.to_string(), asset_server.load(file.as_str()) );
		}
	}
	
	pub fn add( &mut self, name: &str ) -> &mut Self {
		self.files_to_load.push( name.to_string() );
		self
	}
}


fn process(
		mut my_events: EventWriter<LoadingFinished>,
		asset_server: Res<AssetServer>,
		mut query: Query<(&mut LoadingQueue)>,
) {
	for mut loading_queue in query.iter_mut() {
		match ( loading_queue.state ) {
			LoadingState::Init => {
				loading_queue.init_loading( &asset_server );
				loading_queue.state = LoadingState::Loading;
			},
			LoadingState::Loading => {
				if let LoadState::Loaded =
					asset_server.get_group_load_state(loading_queue.handles.values().map(|handle| handle.id))
				{
					loading_queue.state = LoadingState::Finished;
					my_events.send( LoadingFinished );
				}
			},
			LoadingState::Finished => {
			},
		}
	}
}

pub struct LoadingQueuePlugin;

impl Plugin for LoadingQueuePlugin {
	fn build(&self, app: &mut AppBuilder) {
        app
			.add_event::<LoadingFinished>()
			.add_system(process.system());
	}
}

