
use bevy::{
	prelude::*,
	render::pass::ClearColor,
	sprite::collide_aabb::{collide, Collision},
};
use bevy::window::{WindowCreated, WindowResized};


// entity
pub struct ScaleCamera {
	pub window_w : i32,
	pub window_h : i32,
}

impl Default for ScaleCamera {
	fn default() -> Self {
		Self {
			window_w: 0,
			window_h: 0,
		}
	}
}


pub struct ScaleCameraPlugin;

impl Plugin for ScaleCameraPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
			.add_system(on_window_create.system())
			.add_system(on_window_resize.system())
			.add_system(camera_movement_system.system());
	}
}

fn on_window_create(
    created_event: Res<Events<WindowCreated>>,
	mut window: ResMut<WindowDescriptor>,
	windows: Res<Windows>,
	mut query: Query<(&mut ScaleCamera)>,
) {
	let mut event_reader = created_event.get_reader();
	for event in event_reader.iter(&created_event) {
		if let Some(window) = windows.get(event.id) {
			let w = window.width() as i32; //event.width.try_into().unwrap();
			let h = window.height() as i32; //event.height.try_into().unwrap();
			
			for (mut camera) in query.iter_mut() {
				camera.window_w = w;
				camera.window_h = h;
				
				println!("Window created {}x{}", w, h );
			}
		}
	}
}

fn on_window_resize(
	resize_event: Res<Events<WindowResized>>,
	mut window: ResMut<WindowDescriptor>,
	mut query: Query<(&mut ScaleCamera)>,
) {
	let mut event_reader = resize_event.get_reader();
	for event in event_reader.iter(&resize_event) {
		let w = event.width as i32; //event.width.try_into().unwrap();
		let h = event.height as i32; //event.height.try_into().unwrap();
		
		for (mut camera) in query.iter_mut() {
			camera.window_w = w;
			camera.window_h = h;
			
			println!("Window resize {}x{}", w, h );
		}
	}
}

fn camera_movement_system(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&mut ScaleCamera, &mut Transform)>,
) {
	for (mut camera, mut transform) in query.iter_mut() {
		let ratio_w = 2560.0 / camera.window_w as f32;
		let ratio_h = 1440.0 / camera.window_h as f32;
		
		println!("Window ratio {}x{}", ratio_w, ratio_h );
		
		transform.scale = Vec3::new( ratio_w, ratio_h, 1.0 );
		// let (axis_h, axis_v, axis_float) = if options.enabled {
			// (
				// movement_axis(&keyboard_input, options.key_right, options.key_left),
				// movement_axis(
					// &keyboard_input,
					// options.key_backward,
					// options.key_forward,
				// ),
				// movement_axis(&keyboard_input, options.key_up, options.key_down),
			// )
		// } else {
			// (0.0, 0.0, 0.0)
		// };

		// let rotation = transform.rotation;
		// let accel: Vec3 = (strafe_vector(&rotation) * axis_h)
			// + (forward_walk_vector(&rotation) * axis_v)
			// + (Vec3::unit_y() * axis_float);
		// let accel: Vec3 = if accel.length() != 0.0 {
			// accel.normalize() * options.speed
		// } else {
			// Vec3::zero()
		// };

		// let friction: Vec3 = if options.velocity.length() != 0.0 {
			// options.velocity.normalize() * -1.0 * options.friction
		// } else {
			// Vec3::zero()
		// };

		// options.velocity += accel * time.delta_seconds;

		// // clamp within max speed
		// if options.velocity.length() > options.max_speed {
			// options.velocity = options.velocity.normalize() * options.max_speed;
		// }

		// let delta_friction = friction * time.delta_seconds;

		// options.velocity = if (options.velocity + delta_friction).signum()
			// != options.velocity.signum()
		// {
			// Vec3::zero()
		// } else {
			// options.velocity + delta_friction
		// };

		// transform.translation += options.velocity;
	}
}

