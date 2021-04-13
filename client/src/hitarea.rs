use bevy::window::{WindowCreated, WindowResized};
use bevy::{
	input::mouse::MouseButtonInput, input::ElementState, prelude::*,
	window::CursorMoved,
};

struct MouseLoc(Vec2);
struct WindowSize(Vec2);

/// Trait to attach to the camera bundle you wish to track
/// ```
/// .spawn(Camera2dComponents::default())
///		.with(hitarea::HitAreaCamera)
/// ```
pub struct HitAreaCamera;

/// Plugin class that should be added to your app builder
/// ```
/// builder.add_plugin(HitAreaPlugin)
/// ```
pub struct HitAreaPlugin;


impl Plugin for HitAreaPlugin {
	fn build(
		&self,
		app: &mut AppBuilder,
	) {
		app.insert_resource(MouseLoc(Vec2::new(0.0, 0.0)))
			.insert_resource(WindowSize(Vec2::new(0.0, 0.0)))
			.add_system(detect_mouse_event.system())
			.add_system(mouse_movement_updating_system.system())
			.add_system(on_window_create.system())
			.add_system(on_window_resize.system());
	}
}

/// Add this trait to the sprites that you wish to receive click events
/// ```
/// .with(SpritePicker);
/// ```
pub struct SpritePicker;

#[derive(Bundle)] 
pub struct SpritePickerBundle {
	sprite_picker: SpritePicker,
	interaction: Interaction,
}

impl Default for SpritePickerBundle {
	fn default() -> Self {
		SpritePickerBundle {
			sprite_picker: SpritePicker,
			interaction: Interaction::None,
		}
	}
}


pub struct HitArea {
	size: Vec2,
}

impl HitArea {
	#[allow(dead_code)]
	pub fn new(
		size: &Vec2,
	) -> Self {
		HitArea {
			size: size.clone(),
		}
	}
}


#[derive(Bundle)]
pub struct HitAreaBundle {
	hit_area: HitArea,
	interaction: Interaction,
}


impl HitAreaBundle {
	#[allow(dead_code)]
	pub fn new(
		size: &Vec2,
	) -> Self {
		HitAreaBundle {
			hit_area: HitArea {
				size: size.clone(),
			},
			interaction: Interaction::None,
		}
	}
}


fn process_hitarea(
	interaction: &mut Interaction,
	size: &Vec2,
	hitarea_matrix: &Mat4,
	point: &Vec3,
	event: &MouseButtonInput,
) {
	let vec = hitarea_matrix.transform_point3(*point);
	let half_width = size.x / 2.0;
	let half_height = size.y / 2.0;
	if vec.x >= -half_width && vec.x < half_width && vec.y >= -half_height && vec.y < half_height {
		if event.state == ElementState::Pressed && *interaction != Interaction::Clicked {
			*interaction = Interaction::Clicked;
		} else if event.state == ElementState::Released && *interaction == Interaction::Clicked {
			*interaction = Interaction::None;
		}
	}
}

fn detect_mouse_event(
	mouse_pos: ResMut<MouseLoc>,
	window_size: Res<WindowSize>,
	mut my_event_reader: EventReader<MouseButtonInput>,
	mut query_set: QuerySet<(
		Query<(&SpritePicker, &Sprite, &mut Interaction, &GlobalTransform)>,
		Query<(&HitArea, &mut Interaction, &GlobalTransform)>,
		Query<(&HitArea, &mut Interaction), Without<GlobalTransform>>,
	)>,
	camera_query: Query<(&HitAreaCamera, &GlobalTransform)>,
) {
	// move mouse click from 0,0 in bottom left and into the centre of screen
	let point = Vec3::new(
		mouse_pos.0.x - (window_size.0.x / 2.0),
		mouse_pos.0.y - (window_size.0.y / 2.0),
		0.0,
	);

	for event in my_event_reader.iter() {
		for (_camera, camera_transform) in camera_query.iter() {
			let cam_mat = camera_transform.compute_matrix();
			// sprites with SpritePicker type trait
			for (_sprite_picker, sprite, mut interaction, transform) in query_set.q0_mut().iter_mut() {
				let sprite_mat = transform.compute_matrix().inverse() * cam_mat;
				process_hitarea(
					&mut interaction,
					&sprite.size,
					&sprite_mat,
					&point,
					event,
				);
			}
			// HitAreas with GlobalTransform traits
			for (hitarea, mut interaction, transform) in query_set.q1_mut().iter_mut() {
				let sprite_mat = transform.compute_matrix().inverse() * cam_mat;
				process_hitarea(
					&mut interaction,
					&hitarea.size,
					&sprite_mat,
					&point,
					event,
				);
			}
			// Then HitAreas without GlobalTransform trait
			for (hitarea, mut interaction) in query_set.q2_mut().iter_mut() {
				process_hitarea(
					&mut interaction,
					&hitarea.size,
					&cam_mat,
					&point,
					&event,
				);
			}
		}
	}
}

// window create event
fn on_window_create(
	mut created_event: EventReader<WindowCreated>,
	windows: Res<Windows>,
	mut window_size: ResMut<WindowSize>,
) {
	for event in created_event.iter() {
		if let Some(window) = windows.get(event.id) {
			window_size.0.x = window.width() as f32;
			window_size.0.y = window.height() as f32;
		}
	}
}

// window resize event
fn on_window_resize(
	mut resize_event: EventReader<WindowResized>,
	mut _window: ResMut<WindowDescriptor>,
	mut window_size: ResMut<WindowSize>,
) {
	for event in resize_event.iter() {
		window_size.0.x = event.width as f32;
		window_size.0.y = event.height as f32;
	}
}


fn mouse_movement_updating_system(
	mut mouse_pos: ResMut<MouseLoc>,
	mut cursor_moved_events: EventReader<CursorMoved>,
) {
	for event in cursor_moved_events.iter() {
		mouse_pos.0 = event.position;
	}
}


#[cfg(test)]
mod test {}
