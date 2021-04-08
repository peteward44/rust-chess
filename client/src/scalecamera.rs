use bevy::prelude::*;
use bevy::window::{WindowCreated, WindowResized};

// constants
pub const DRAW_WINDOW_W: f32 = 2560.0;
pub const DRAW_WINDOW_H: f32 = 1440.0;
pub const ASPECT_RATIO: f32 = DRAW_WINDOW_W / DRAW_WINDOW_H;

// entity
pub struct ScaleCamera {
	pub window_w: i32,
	pub window_h: i32,
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
		app.add_system(on_window_create.system())
			.add_system(on_window_resize.system())
			.add_system(camera_movement_system.system());
	}
}

fn on_window_create(
	mut created_event: EventReader<WindowCreated>,
	windows: Res<Windows>,
	mut query: Query<&mut ScaleCamera>,
) {
	for event in created_event.iter() {
		if let Some(window) = windows.get(event.id) {
			let w = window.width() as i32;
			let h = window.height() as i32;

			for mut camera in query.iter_mut() {
				camera.window_w = w;
				camera.window_h = h;
			}
		}
	}
}

fn on_window_resize(
	mut resize_event: EventReader<WindowResized>,
	mut _window: ResMut<WindowDescriptor>,
	mut query: Query<&mut ScaleCamera>,
) {
	for event in resize_event.iter() {
		let w = event.width as i32;
		let h = event.height as i32;

		for mut camera in query.iter_mut() {
			camera.window_w = w;
			camera.window_h = h;
		}
	}
}

fn scale_basic(window_w: i32, window_h: i32) -> Vec3 {
	// plain simple scale direct to window size
	let ratio_w = DRAW_WINDOW_W / window_w as f32;
	let ratio_h = DRAW_WINDOW_H / window_h as f32;
	Vec3::new(ratio_w, ratio_h, 1.0)
}

fn scale_maintain_aspect_ratio(window_w: i32, window_h: i32) -> Vec3 {
	let mut ratio_w = DRAW_WINDOW_W / window_w as f32;
	let desired_height = window_w as f32 / ASPECT_RATIO;
	let ratio_h;

	// if window isn't tall enough to fit in desired height, reduce the size of the width instead
	if desired_height > window_h as f32 {
		ratio_h = DRAW_WINDOW_H / window_h as f32;
		let desired_width = window_h as f32 * ASPECT_RATIO;
		ratio_w = DRAW_WINDOW_W / desired_width;
	} else {
		ratio_h = DRAW_WINDOW_H / desired_height;
	}
	Vec3::new(ratio_w, ratio_h, 1.0)
}

fn camera_movement_system(mut query: Query<(&ScaleCamera, &mut Transform)>) {
	for (camera, mut transform) in query.iter_mut() {
		transform.scale = scale_maintain_aspect_ratio(camera.window_w, camera.window_h);
	}
}
