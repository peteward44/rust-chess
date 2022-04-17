use crate::boardstate::BoardState;
use crate::consts;
use crate::hitarea::SpritePickerBundle;
use crate::rules::Rules;
use bevy::prelude::*;

// classes
#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
struct Square {
	color: Color,
}

#[derive(Component, Debug, Clone)]
struct SquarePosition {
	x: i32,
	y: i32,
}

// resources
struct BoardRenderState {
	selected: Option<SquarePosition>,
	squares: [[Square; consts::BOARD_WIDTH as usize]; consts::BOARD_HEIGHT as usize],
}

// events
pub struct PieceMoved;


fn get_square_color(
	x: i32,
	y: i32,
) -> Color {
	let white = Color::rgb(consts::BOARD_COLOUR1.0, consts::BOARD_COLOUR1.1, consts::BOARD_COLOUR1.2);
	let black = Color::rgb(consts::BOARD_COLOUR2.0, consts::BOARD_COLOUR2.1, consts::BOARD_COLOUR2.2);
	if ((y as i32 % 2) - (x as i32 % 2)) == 0 {
		return black;
	}
	return white;
}

fn on_enter(
	mut commands: Commands,
	board_render_state: ResMut<BoardRenderState>,
	_asset_server: Res<AssetServer>,
) {
	for y in 0..consts::BOARD_WIDTH {
		for x in 0..consts::BOARD_HEIGHT {
			let square_render = &board_render_state.squares[x as usize][y as usize];
			let pos = consts::get_square_position(x, y);
			commands
				.spawn_bundle(SpriteBundle {
					transform: Transform::from_translation(Vec3::new(pos.0, pos.1, 0.0)),
					sprite: Sprite {
						custom_size: Some(Vec2::new(consts::SQUARE_WIDTH, consts::SQUARE_HEIGHT)),
						color: square_render.color,
						..Default::default()
					},
					..Default::default()
				})
				.insert_bundle(SpritePickerBundle::default())
				.insert(SquarePosition { x: x, y: y });
		}
	}
}

fn square_clicked(
	mut board_render_state: ResMut<BoardRenderState>,
	board_state: Res<BoardState>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut interaction_query: Query<(&Interaction, &SquarePosition, &mut Sprite), (Changed<Interaction>, With<SquarePosition>, With<Sprite>)>,
) {
	for (interaction, square, mut sprite) in interaction_query.iter_mut() {
		// let square = square_query.get_mut(children[0]).unwrap();
		match *interaction {
			Interaction::Clicked => {
				println!("Clicked {:?} {:?}", square.x, square.y);
				if Rules::is_occupied(&board_state, square.x, square.y) {
					let mut is_same = false;
					let selected = board_render_state.selected.clone();
					match selected {
						Some(selected_square) => {
							// reset already-selected square to original colour
							board_render_state.squares[selected_square.x as usize][selected_square.y as usize].color = get_square_color(selected_square.x, selected_square.y);
						//	let mut color_mat = materials.get_mut(&square_render.material).unwrap();
						//	color_mat.color = get_square_color(selected_square.x, selected_square.y);
							is_same = selected_square.x == square.x && selected_square.y == square.y;
						}
						None => {}
					}
					// set newly selected square to selected colour
					if !is_same {
						board_render_state.squares[square.x as usize][square.y as usize].color = Color::rgb(1.0, 1.0, 1.0);
						sprite.color = Color::rgb(1.0, 1.0, 1.0);
					//	let mut color_mat = materials.get_mut(&square_render.material).unwrap();
					//	color_mat.color = Color::rgb(1.0, 1.0, 1.0);
						board_render_state.selected = Some((*square).clone());

						// highlight squares available to move to
						let piece = board_state.get_square(square.x, square.y);
						match piece {
							Some(board_piece) => {
								let possible_moves = Rules::get_piece_possible_moves(&board_state, square.x, square.y, &board_piece);
								println!("Move: {:?}", board_piece.piece);
								for pmove in possible_moves.iter() {
									// change colour of potential move squares
									board_render_state.squares[pmove.x as usize][pmove.y as usize].color = Color::rgb(0.5, 0.5, 0.5).into();
								//	let mut color_mat = materials.get_mut(&pmove_square_render.material).unwrap();
								//	color_mat.color = Color::rgb(0.5, 0.5, 0.5);
									println!("Possible move: {:?} {:?}", pmove.x, pmove.y);
								}
							}
							_ => {}
						}
					} else {
						board_render_state.selected = None;
					}
				}
			}
			_ => {
				//		println!("Something else {:?} {:?}", square.x, square.y);
			}
		}
	}
}


fn on_exit(
	mut commands: Commands,
	mut query: Query<(&Square, Entity)>,
) {
	for (_square, entity) in query.iter_mut() {
		commands.entity(entity).despawn_recursive();
	}
}


fn escape_key(
	keys: Res<Input<KeyCode>>,
	mut state: ResMut<State<consts::GameState>>,
) {
	if keys.just_pressed(KeyCode::Escape) {
		state.set(consts::GameState::Menu).unwrap();
	}
}


fn prep_board(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let squares: [[Square; consts::BOARD_WIDTH as usize]; consts::BOARD_HEIGHT as usize] = array_init::array_init(|x: usize| {
		array_init::array_init(|y: usize| {
			let color = get_square_color(x as i32, y as i32);
			Square { color: color.into() }
		//	let material = materials.add(colour.into());
		//	Square { material: material }
		})
	});
	commands.insert_resource(BoardRenderState {
		selected: None,
		squares: squares,
	});
}


// Plugin
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
	fn build(
		&self,
		app: &mut App,
	) {
		app.add_event::<PieceMoved>()
			.add_startup_system(prep_board.system())
			.add_system_set(SystemSet::on_enter(consts::GameState::Playing).with_system(on_enter.system()))
			.add_system_set(
				SystemSet::on_update(consts::GameState::Playing)
					.with_system(square_clicked.system())
					.with_system(escape_key.system()),
			)
			.add_system_set(SystemSet::on_exit(consts::GameState::Playing).with_system(on_exit.system()));
	}
}
