use bevy::{
	prelude::*,
	render::pass::ClearColor,
	sprite::collide_aabb::{collide, Collision},
};


pub struct InputPlugin;


impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
		app.add_system(player_movement_system.system());
    }
}



fn player_movement_system(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
//	client: ResMut<Mutex<websocket::sync::Client<std::net::TcpStream>>>,
//	mut query: Query<(&Player, &mut Transform)>,
) {
	// for (player, mut transform) in query.iter_mut() {
		// let mut directionx = 0.0;
		// let mut directiony = 0.0;
		// let mut command = "";

		// if keyboard_input.pressed(KeyCode::Left) {
			// directionx -= 1.0;
			// command = "left";
		// }
		// if keyboard_input.pressed(KeyCode::Right) {
			// directionx += 1.0;
			// command = "right";
		// }
		// if keyboard_input.pressed(KeyCode::Down) {
			// directiony -= 1.0;
			// command = "down";
		// }
		// if keyboard_input.pressed(KeyCode::Up) {
			// directiony += 1.0;
			// command = "up";
		// }

		// if ( command.len() > 0 ) {
			// let translation = &mut transform.translation;

			// *translation.x_mut() += time.delta_seconds * directionx * 500.0;
			// *translation.x_mut() = translation.x().min(380.0).max(-380.0);

			// *translation.y_mut() += time.delta_seconds * directiony * 500.0;
			// *translation.y_mut() = translation.y().min(380.0).max(-380.0);

			// let message = Message::text(command);
			// client.lock().unwrap().send_message(&message).unwrap(); // Send message
		// }
	// }
}
