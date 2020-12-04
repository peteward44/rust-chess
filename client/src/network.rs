extern crate websocket;

use bevy::{
	prelude::*,
	render::pass::ClearColor,
	sprite::collide_aabb::{collide, Collision},
};
use websocket::{ClientBuilder, Message};
//use websocket::client::Client;



pub struct NetworkPlugin;


impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
		app.add_startup_system(network_setup.system());
    }
}


fn network_setup(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>
) {
	// let mut client = ClientBuilder::new("ws://127.0.0.1:8080")
		// .unwrap()
		// .connect_insecure()
		// .unwrap();
	// let res = Mutex::new( client );
	// commands.insert_resource( res );
}
