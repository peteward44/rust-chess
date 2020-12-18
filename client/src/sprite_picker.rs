use std::fmt;
use bevy::window::{WindowCreated, WindowResized};
use bevy::{
	input::mouse::MouseButton, input::mouse::MouseButtonInput, input::ElementState, prelude::*,
	window::CursorMoved,
};

#[derive(Default)]
struct State {
	mouse_button_event_reader: EventReader<MouseButtonInput>,
	cursor_moved_event_reader: EventReader<CursorMoved>,
}
struct MouseLoc(Vec2);
struct WindowSize(Vec2);

// trait to attach to camera bundle
pub struct SpritePickerCamera;

// plugin
pub struct SpritePickerPlugin;

// event
#[derive(Debug)]
pub struct MouseClick {
	name: String,
	button: MouseButton,
	state: ElementState,
}


impl Plugin for SpritePickerPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_resource(MouseLoc(Vec2::new(0.0, 0.0)))
			.add_resource(WindowSize(Vec2::new(0.0, 0.0)))
			.add_event::<MouseClick>()
			.add_system(detect_mouse_event.system())
			.add_system(mouse_movement_updating_system.system())
			.add_system(on_window_create.system())
			.add_system(on_window_resize.system());
	}
}

// trait to add to sprites
pub struct SpritePicker {
	name: String,
}

// impl fmt::Display for SpritePicker {
    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{}", self.name)
    // }
// }


impl SpritePicker {
	pub fn new(name: &str) -> Self {
		SpritePicker {
			name: name.to_owned(),
		}
	}
}

fn detect_mouse_event(
	mut state: Local<State>,
	mouse_pos: ResMut<MouseLoc>,
	window_size: Res<WindowSize>,
	mut my_events: ResMut<Events<MouseClick>>,
	mouse_button_input_events: Res<Events<MouseButtonInput>>,
	query: Query<(&SpritePicker, &Sprite, &GlobalTransform)>,
	camera_query: Query<(&SpritePickerCamera, &Transform)>,
) {
	// move mouse click from 0,0 in bottom left and into the centre of screen
	let point = Vec3::new(
		mouse_pos.0.x() - (window_size.0.x() / 2.0),
		mouse_pos.0.y() - (window_size.0.y() / 2.0),
		0.0,
	);

	for event in state
		.mouse_button_event_reader
		.iter(&mouse_button_input_events)
	{
		for (_camera, camera_transform) in camera_query.iter() {
			let cam_mat = camera_transform.compute_matrix();

			for (sprite_picker, sprite, sprite_transform) in query.iter() {
				let sprite_mat = sprite_transform.compute_matrix();
				let final_mat = sprite_mat * cam_mat;
				let vec = final_mat.transform_point3(point);

				let half_width = sprite.size.x() / 2.0;
				let half_height = sprite.size.y() / 2.0;
				if vec.x() >= -half_width
					&& vec.x() < half_width
					&& vec.y() >= -half_height
					&& vec.y() < half_height
				{
					println!(
						"sprite clicked! event: {:?} position: {:?}",
						event, mouse_pos.0
					);
					my_events.send(MouseClick {
						name: sprite_picker.name.clone(),
						button: event.button,
						state: event.state.clone(),
					});
				}
			}
		}
	}
}

fn mouse_movement_updating_system(
	mut state: Local<State>,
	mut mouse_pos: ResMut<MouseLoc>,
	cursor_moved_events: Res<Events<CursorMoved>>,
) {
	for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
		mouse_pos.0 = event.position;
	}
}

// window create event
fn on_window_create(
	created_event: Res<Events<WindowCreated>>,
	windows: Res<Windows>,
	mut window_size: ResMut<WindowSize>,
) {
	let mut event_reader = created_event.get_reader();
	for event in event_reader.iter(&created_event) {
		if let Some(window) = windows.get(event.id) {
			window_size.0.set_x(window.width() as f32);
			window_size.0.set_y(window.height() as f32);
		}
	}
}

// window resize event
fn on_window_resize(
	resize_event: Res<Events<WindowResized>>,
	mut _window: ResMut<WindowDescriptor>,
	mut window_size: ResMut<WindowSize>,
) {
	let mut event_reader = resize_event.get_reader();
	for event in event_reader.iter(&resize_event) {
		window_size.0.set_x(event.width as f32);
		window_size.0.set_y(event.height as f32);
	}
}
