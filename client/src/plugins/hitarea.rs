use bevy::window::{WindowCreated, WindowResized};
use bevy::{input::mouse::MouseButtonInput, input::ButtonState, prelude::*, window::CursorMoved};

#[derive(Resource)]
struct MouseLoc(Vec2);

#[derive(Resource)]
struct WindowSize(Vec2);

/// Trait to attach to the camera bundle you wish to track
/// ```
/// .spawn(Camera2dComponents::default())
///		.with(hitarea::HitAreaCamera)
/// ```
#[derive(Component)]
pub struct HitAreaCamera;

/// Plugin class that should be added to your app builder
/// ```
/// builder.add_plugin(HitAreaPlugin)
/// ```
pub struct HitAreaPlugin;

pub struct InteractionEvent {
	pub entity: Entity,
    pub button: MouseButton,
    pub state: ButtonState,
}


impl Plugin for HitAreaPlugin {
	fn build(
		&self,
		app: &mut App,
	) {
		app.insert_resource(MouseLoc(Vec2::new(0.0, 0.0)))
			.insert_resource(WindowSize(Vec2::new(0.0, 0.0)))
			.add_event::<InteractionEvent>()
			.add_system(detect_mouse_event)
			.add_system(mouse_movement_updating_system)
			.add_system(on_window_create)
			.add_system(on_window_resize);
	}
}

/// Add this trait to the sprites that you wish to receive click events
/// ```
/// .with(SpritePicker);
/// ```
#[derive(Component)]
pub struct SpritePicker;

#[derive(Bundle)]
pub struct SpritePickerBundle {
	sprite_picker: SpritePicker,
}

impl Default for SpritePickerBundle {
	fn default() -> Self {
		SpritePickerBundle {
			sprite_picker: SpritePicker,
		}
	}
}

#[derive(Component)]
pub struct HitArea {
	size: Vec2,
}

impl HitArea {
	#[allow(dead_code)]
	pub fn new(size: &Vec2) -> Self {
		HitArea { size: size.clone() }
	}
}


#[derive(Bundle)]
pub struct HitAreaBundle {
	hit_area: HitArea,
}


impl HitAreaBundle {
	#[allow(dead_code)]
	pub fn new(size: &Vec2) -> Self {
		HitAreaBundle {
			hit_area: HitArea { size: size.clone() },
		}
	}
}

fn process_hitarea(
	entity: Entity,
	my_event_writer: &mut EventWriter<InteractionEvent>,
	event: &MouseButtonInput,
	size: &Vec2,
	hitarea_matrix: &Mat4,
	point: &Vec3,
) {
	let vec = hitarea_matrix.transform_point3(*point);
	let half_width = size.x / 2.0;
	let half_height = size.y / 2.0;
	if vec.x >= -half_width && vec.x < half_width && vec.y >= -half_height && vec.y < half_height {
		my_event_writer.send(InteractionEvent{ entity: entity, button: event.button, state: event.state})
	}
}

fn detect_mouse_event(
	mouse_pos: ResMut<MouseLoc>,
	window_size: Res<WindowSize>,
	mut my_event_reader: EventReader<MouseButtonInput>,
	mut my_event_writer: EventWriter<InteractionEvent>,
	mut query_set: ParamSet<(
		Query<(Entity, &SpritePicker, &Sprite, &GlobalTransform)>,
		Query<(Entity, &HitArea, &GlobalTransform)>,
		Query<(Entity, &HitArea), Without<GlobalTransform>>,
	)>,
	camera_query: Query<(&HitAreaCamera, &GlobalTransform)>,
) {
	// move mouse click from 0,0 in bottom left and into the centre of screen
	let point = Vec3::new(mouse_pos.0.x - (window_size.0.x / 2.0), mouse_pos.0.y - (window_size.0.y / 2.0), 0.0);

	for event in my_event_reader.iter() {
		for (_camera, camera_transform) in camera_query.iter() {
			let cam_mat = camera_transform.compute_matrix();
			// sprites with SpritePicker type trait
			for (entity, _sprite_picker, sprite, transform) in query_set.p0().iter_mut() {
				match sprite.custom_size {
					Some(size) => {
						let sprite_mat = transform.compute_matrix().inverse() * cam_mat;
						process_hitarea(entity, &mut my_event_writer, &event, &size, &sprite_mat, &point);
					},
					None => {
					}
				}
			}
			// HitAreas with GlobalTransform traits
			for (entity, hitarea, transform) in query_set.p1().iter_mut() {
				let sprite_mat = transform.compute_matrix().inverse() * cam_mat;
				process_hitarea(entity, &mut my_event_writer, &event, &hitarea.size, &sprite_mat, &point);
			}
			// Then HitAreas without GlobalTransform trait
			for (entity, hitarea) in query_set.p2().iter_mut() {
				process_hitarea(entity, &mut my_event_writer, &event, &hitarea.size, &cam_mat, &point);
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
