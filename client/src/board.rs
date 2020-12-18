use super::consts;
use super::scalecamera::ScaleCamera;
use super::sprite_picker::{SpritePicker, MouseClick};
use bevy::{input::mouse::MouseButtonInput, prelude::*, window::CursorMoved};

// classes
#[allow(dead_code)]
struct Square {
	x: i32,
	y: i32,
}

#[derive(Default)]
pub struct State {
	// Set up from example
	mouse_button_event_reader: EventReader<MouseButtonInput>,
	cursor_moved_event_reader: EventReader<CursorMoved>,
}
pub struct MouseLoc(Vec2);

// events
pub struct PieceMoved;

// entity
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_event::<PieceMoved>()
			.add_startup_system(startup.system())
			.add_resource(MouseLoc(Vec2::new(0.0, 0.0)))
			.add_system(mouse_movement_updating_system.system())
			.add_system(select_square.system())
			.add_system(square_clicked.system());
	}
}

fn startup(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	_asset_server: Res<AssetServer>,
) {
	let white = Color::rgb(
		consts::BOARD_COLOUR1.0,
		consts::BOARD_COLOUR1.1,
		consts::BOARD_COLOUR1.2,
	);
	let black = Color::rgb(
		consts::BOARD_COLOUR2.0,
		consts::BOARD_COLOUR2.1,
		consts::BOARD_COLOUR2.2,
	);
	for y in 0..consts::BOARD_WIDTH {
		for x in 0..consts::BOARD_HEIGHT {
			let mut colour = white;
			if ((y % 2) - (x % 2)) == 0 {
				colour = black;
			}
			let pos = consts::get_square_position(x, y);
			commands
				.spawn(SpriteComponents {
					material: materials.add(colour.into()),
					transform: Transform::from_translation(Vec3::new(pos.0, pos.1, 0.0)),
					sprite: Sprite::new(Vec2::new(consts::SQUARE_WIDTH, consts::SQUARE_HEIGHT)),
					..Default::default()
				})
				.with(Square { x: x, y: y })
				.with(SpritePicker::new(&format!("{} {}", x, y)));
		}
	}
}

fn square_clicked(
    mut my_event_reader: Local<EventReader<MouseClick>>,
    my_events: Res<Events<MouseClick>>,
) {
    for my_event in my_event_reader.iter(&my_events) {
        println!("{:?}", my_event);
    }
}

fn select_square(
	mut state: Local<State>,
	mouse_pos: ResMut<MouseLoc>,
	mouse_button_input_events: Res<Events<MouseButtonInput>>,
	cameraq: Query<&ScaleCamera>,
) {
	for event in state
		.mouse_button_event_reader
		.iter(&mouse_button_input_events)
	{
		//println!("event: {:?} position: {:?}", event, mouse_pos.0);
		let camera = cameraq.iter().next().unwrap();
		let maybe_pos = camera.position_to_drawing_area(&mouse_pos.0);
		match maybe_pos {
			None => (),
			Some(pos) => {
				//		println!("drawing position: {:?}", pos);
			}
		}
	}
}

fn mouse_movement_updating_system(
	mut mouse_pos: ResMut<MouseLoc>,
	mut state: Local<State>,
	cursor_moved_events: Res<Events<CursorMoved>>,
) {
	for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
		mouse_pos.0 = event.position;
	}
}

// fn touch_system(touches: Res<Touches>) {
// for touch in touches.iter_just_pressed() {
// println!(
// "just pressed touch with id: {:?}, at: {:?}",
// touch.id, touch.position
// );
// }

// for touch in touches.iter_just_released() {
// println!(
// "just released touch with id: {:?}, at: {:?}",
// touch.id, touch.position
// );
// }

// for touch in touches.iter_just_cancelled() {
// println!("cancelled touch with id: {:?}", touch.id);
// }

// // you can also iterate all current touches and retrieve their state like this:
// for touch in touches.iter() {
// println!("active touch: {:?}", touch);
// println!("  just_pressed: {}", touches.just_pressed(touch.id));
// }
// }

//fn system(_my_events: Res<Events<PieceMoved>>) {}
// fn system(
// time: Res<Time>,
// keyboard_input: Res<Input<KeyCode>>,
// client: ResMut<Mutex<websocket::sync::Client<std::net::TcpStream>>>,
// mut query: Query<(&Player, &mut Transform)>,
// ) {

// }
