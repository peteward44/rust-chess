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

/// Trait to attach to the camera bundle you wish to track
/// ```
/// .spawn(Camera2dComponents::default())
///		.with(sprite_picker::SpritePickerCamera)
/// ```
pub struct SpritePickerCamera;


/// Plugin class that should be added to your app builder
/// ```
/// builder.add_plugin(SpritePickerPlugin)
/// ```
pub struct SpritePickerPlugin;

/// Mouse click event
/// "name" is the name passed to the SpritePicker trait, "button" and "state" are the same from bevy's
/// event system, and "pos" is the relative position in the hit area that the click occurred, from the centre of the area.
#[derive(Debug)]
pub struct MouseClick {
	name: String,
	button: MouseButton,
	state: ElementState,
	pos: Vec2,
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

/// Add this trait to the sprites that you wish to receive click events
/// ```
/// .with(SpritePicker::new("my_sprite"));
/// ```
pub struct SpritePicker {
	name: String,
}

impl SpritePicker {
	pub fn new(name: &str) -> Self {
		SpritePicker {
			name: name.to_owned(),
		}
	}
}


pub struct HitArea {
	name: String,
	size: Vec2,
}

impl HitArea {
	pub fn new(name: &str, size: &Vec2) -> Self {
		HitArea {
			name: name.to_owned(),
			size: size.clone(),
		}
	}
}


fn process_hitarea(
	name: &String,
	size: &Vec2,
	hitarea_matrix: &Mat4,
	point: &Vec3,
	event: &MouseButtonInput,
	my_events: &mut ResMut<Events<MouseClick>>,
) {
	let vec = hitarea_matrix.transform_point3(*point);

	let half_width = size.x / 2.0;
	let half_height = size.y / 2.0;
	if vec.x >= -half_width
		&& vec.x < half_width
		&& vec.y >= -half_height
		&& vec.y < half_height
	{
		my_events.send(MouseClick {
			name: name.to_string(),
			button: event.button,
			state: event.state.clone(),
			pos: Vec2::new( vec.x, vec.y ),
		});
	}
}


fn detect_mouse_event(
	mut state: Local<State>,
	mouse_pos: ResMut<MouseLoc>,
	window_size: Res<WindowSize>,
	mut my_events: ResMut<Events<MouseClick>>,
	mouse_button_input_events: Res<Events<MouseButtonInput>>,
	query: Query<(&SpritePicker, &Sprite, &GlobalTransform)>,
	hitarea_query: Query<(&HitArea, &GlobalTransform)>,
	hitarea_notransform_query: Query<&HitArea, Without<GlobalTransform>>,
	camera_query: Query<(&SpritePickerCamera, &GlobalTransform)>,
) {
	// move mouse click from 0,0 in bottom left and into the centre of screen
	let point = Vec3::new(
		mouse_pos.0.x - (window_size.0.x / 2.0),
		mouse_pos.0.y - (window_size.0.y / 2.0),
		0.0,
	);

	for event in state
		.mouse_button_event_reader
		.iter(&mouse_button_input_events)
	{
		for (_camera, camera_transform) in camera_query.iter() {
			let cam_mat = camera_transform.compute_matrix();
			// sprites with SpritePicker type trait
			for (sprite_picker, sprite, transform) in query.iter() {
				let sprite_mat = transform.compute_matrix().inverse() * cam_mat;
				process_hitarea( &sprite_picker.name, &sprite.size, &sprite_mat, &point, &event, &mut my_events );
			}
			// HitAreas with GlobalTransform traits
			for (hitarea, transform) in hitarea_query.iter() {
				let sprite_mat = transform.compute_matrix().inverse() * cam_mat;
				process_hitarea( &hitarea.name, &hitarea.size, &sprite_mat, &point, &event, &mut my_events );
			}
			// Then HitAreas without GlobalTransform trait
			for hitarea in hitarea_notransform_query.iter() {
				process_hitarea( &hitarea.name, &hitarea.size, &cam_mat, &point, &event, &mut my_events );
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
			window_size.0.x = window.width() as f32;
			window_size.0.y = window.height() as f32;
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
		window_size.0.x = event.width as f32;
		window_size.0.y = event.height as f32;
	}
}
