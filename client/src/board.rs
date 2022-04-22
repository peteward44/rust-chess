use crate::boardstate::BoardState;
use crate::consts;
use crate::hitarea::SpritePickerBundle;
use crate::rules::Rules;
use crate::boardstate::BoardPiece;
use bevy::prelude::*;
use std::collections::HashMap;
use std::ops::{Add, Mul};

// classes
#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
struct Square {
	color: Color,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct SquarePosition {
	x: i32,
	y: i32,
}

impl Add for SquarePosition {
    type Output = SquarePosition;

    fn add(self, rhs: Self) -> Self::Output {
        Self{ x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Mul<i32> for SquarePosition {
    type Output = SquarePosition;

    fn mul(self, rhs: i32) -> Self::Output {
        Self{ x: self.x * rhs, y: self.y * rhs }
    }
}

#[derive(Component, Debug, Clone)]
pub struct MoveTo {
	from: SquarePosition,
	to: SquarePosition,
}


// modifiers
#[derive(Component, Debug, Clone)]
enum SquareState {
	None,
	Selected,
	PossibleMove,
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
	mut square_entities: ResMut<HashMap<SquarePosition, Entity>>,
	_asset_server: Res<AssetServer>,
) {
	for y in 0..consts::BOARD_WIDTH {
		for x in 0..consts::BOARD_HEIGHT {
			let square_render = &board_render_state.squares[x as usize][y as usize];
			let pos = consts::get_square_position(x, y);
			let entity = commands
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
				.insert(SquarePosition { x: x, y: y })
				.insert(SquareState::None)
				.id();
			square_entities.insert(SquarePosition { x: x, y: y }, entity);
		}
	}
}

fn square_selected_changed(
	mut selected_query: Query<(&SquareState, &SquarePosition, &mut Sprite), (Changed<SquareState>, With<SquarePosition>, With<Sprite>)>,
) {
	for (square_state, square, mut sprite) in selected_query.iter_mut() {
		match *square_state {
			SquareState::None => {
				sprite.color = get_square_color(square.x, square.y);
			},
			SquareState::Selected => {
				sprite.color = Color::rgb(1.0, 1.0, 1.0).into();
			},
			SquareState::PossibleMove => {
				sprite.color = Color::rgb(0.5, 0.5, 0.5).into();
			},
		}
	}
}

fn square_clicked(
	mut commands: Commands,
	mut board_render_state: ResMut<BoardRenderState>,
	board_state: Res<BoardState>,
	square_entities: ResMut<HashMap<SquarePosition, Entity>>,
	piece_map: ResMut<consts::PieceMap>,
	mut interaction_query: Query<(Entity, &Interaction, &SquarePosition), (Changed<Interaction>, With<SquarePosition>)>,
) {
	for (entity, interaction, square) in interaction_query.iter_mut() {
		match *interaction {
			Interaction::Clicked => {
				println!("Clicked {:?} {:?}", square.x, square.y);
				if Rules::is_occupied(&board_state, square.x, square.y) {
					let mut is_same = false;
					let selected = board_render_state.selected.clone();
					match selected {
						Some(selected_square) => {
							is_same = selected_square.x == square.x && selected_square.y == square.y;
						}
						None => {}
					}
					// reset all square states
					for entity in square_entities.values() {
						commands.entity(*entity).insert(SquareState::None);
					}
					// set newly selected square to selected colour
					if !is_same {
						commands.entity(entity).insert(SquareState::Selected);
						board_render_state.selected = Some((*square).clone());

						// highlight squares available to move to
						let piece = board_state.get_piece_by_position(square.x, square.y);
						match piece {
							Some(board_piece) => {
								let possible_moves = Rules::get_piece_possible_moves(&board_state, square.x, square.y, &board_piece);
								println!("Move: {:?}", board_piece.piece);
								for pmove in possible_moves.iter() {
									// change colour of potential move squares
									let ent = *square_entities.get(&SquarePosition{x: pmove.x, y: pmove.y}).unwrap();
									commands.entity(ent).insert(SquareState::PossibleMove);
									println!("Possible move: {:?} {:?}", pmove.x, pmove.y);
								}
							}
							_ => {}
						}
					} else {
						board_render_state.selected = None;
					}
				} else {
					let selected = board_render_state.selected.clone();
					match selected {
						Some(selected_square) => {
							let piece = board_state.get_piece_by_position(selected_square.x, selected_square.y);
							match piece {
								Some(board_piece) => {
									let possible_moves = Rules::get_piece_possible_moves(&board_state, selected_square.x, selected_square.y, &board_piece);
									// see if clicked square is a possible move
									println!("Move: {:?}", board_piece.piece);
									let found = possible_moves.iter().find(|m| m.x == square.x && m.y == square.y);
									match found {
										Some(_found_move) => {
											println!("Valid move: {:?}", board_piece.piece);
											let ent = *piece_map.get(&board_piece.id).unwrap();
											commands.entity(ent).insert(MoveTo{from: selected_square, to: *square});
										}
										_ => {}
									}
								}
								_ => {}
							}
						}
						None => {}
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
) {
	let squares: [[Square; consts::BOARD_WIDTH as usize]; consts::BOARD_HEIGHT as usize] = array_init::array_init(|x: usize| {
		array_init::array_init(|y: usize| {
			let color = get_square_color(x as i32, y as i32);
			Square { color: color.into() }
		})
	});
	commands.insert_resource(BoardRenderState {
		selected: None,
		squares: squares,
	});
}


fn on_piece_moveto(
	mut commands: Commands,
	mut board_render_state: ResMut<BoardRenderState>,
	board_state: Res<BoardState>,
	square_entities: ResMut<HashMap<SquarePosition, Entity>>,
	piece_map: ResMut<consts::PieceMap>,
	mut moveto_query: Query<(Entity, &MoveTo, &BoardPiece), (Changed<MoveTo>, With<BoardPiece>)>,
) {
	for (entity, moveto, board_piece) in moveto_query.iter_mut() {
		println!("Move: {0}x{1}", moveto.from.x, moveto.from.y);
	}	
}



// Plugin
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
	fn build(
		&self,
		app: &mut App,
	) {
		app.insert_resource(HashMap::<SquarePosition, Entity>::new())
			.add_event::<PieceMoved>()
			.add_startup_system(prep_board)
			.add_system_set(SystemSet::on_enter(consts::GameState::Playing).with_system(on_enter))
			.add_system_set(
				SystemSet::on_update(consts::GameState::Playing)
					.with_system(square_clicked)
					.with_system(escape_key)
					.with_system(on_piece_moveto)
					.with_system(square_selected_changed),
			)
			.add_system_set(SystemSet::on_exit(consts::GameState::Playing).with_system(on_exit));
	}
}
