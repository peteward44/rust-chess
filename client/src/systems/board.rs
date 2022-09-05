use crate::consts;
use crate::components;
use crate::resources;
use crate::plugins;
use bevy::input::*;
use bevy::prelude::*;
use shakmaty::{Position};

pub fn on_startup(
	mut commands: Commands,
	mut board_render_state: ResMut<resources::board_renderstate::BoardRenderState>,
	mut board_piece_state: ResMut<resources::board_piecestate::BoardPieceState>,
	chess: Res<shakmaty::Chess>,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	board_render_state.init(&mut commands);
	// board_piece_state.new_game_setup(true);
	
	// let legals = chess.legal_moves();
	// for mv in legals.iter() {
	// 	println!("{:?}", mv);
	// }

	board_piece_state.spawn_ecs_components_shakmaty(&mut commands, asset_server, texture_atlases, chess);
}

pub fn on_enter(
	mut commands: Commands,
	mut board_render_state: ResMut<resources::board_renderstate::BoardRenderState>,
) {
	board_render_state.reset_all(&mut commands);
}

pub fn change_square_colour_on_selected_change(
	mut selected_query: Query<(&components::board::SquareSelectedState, &components::board::SquarePosition, &components::board::SquarePossibleMoveState, &mut Sprite), (Changed<components::board::SquareSelectedState>, With<components::board::SquarePosition>, With<components::board::SquarePossibleMoveState>, With<Sprite>)>,
	board_render_state: Res<resources::board_renderstate::BoardRenderState>,
) {
	for (selected_state, square, possible_move_state, mut sprite) in selected_query.iter_mut() {
		sprite.color = board_render_state.get_square_color(square.x, square.y, selected_state.clone(), possible_move_state.clone());
	}
}

pub fn change_square_colour_on_possible_move_change(
	mut selected_query: Query<(&components::board::SquarePossibleMoveState, &components::board::SquarePosition, &components::board::SquareSelectedState, &mut Sprite), (Changed<components::board::SquarePossibleMoveState>, With<components::board::SquarePosition>, With<components::board::SquareSelectedState>, With<Sprite>)>,
	board_render_state: Res<resources::board_renderstate::BoardRenderState>,
) {
	for (possible_move_state, square, selected_state, mut sprite) in selected_query.iter_mut() {
		sprite.color = board_render_state.get_square_color(square.x, square.y, selected_state.clone(), possible_move_state.clone());
	}
}

pub fn show_possible_moves_on_state_change(
	mut commands: Commands,
	mut selected_query: Query<(&components::board::SquareSelectedState, &components::board::SquarePosition, &mut Sprite), (Changed<components::board::SquareSelectedState>, With<components::board::SquarePosition>, With<Sprite>)>,
	board_piece_state: Res<resources::board_piecestate::BoardPieceState>,
	mut board_render_state: ResMut<resources::board_renderstate::BoardRenderState>,
) {
	for (square_state, square, mut sprite) in selected_query.iter_mut() {
		match *square_state {
			components::board::SquareSelectedState::Selected => {
				// let possible_moves = board_piece_state.get_possible_moves(pmove.x, pmove.y);
				// for pmove in possible_moves.iter() {
				// 	// change colour of potential move squares
				// 	board_render_state.set_possible_move(&mut commands, pmove.x, pmove.y);
				// 	println!("Possible move: {:?} {:?}", pmove.x, pmove.y);
				// }
			},
			components::board::SquareSelectedState::None => {

			},
		}
	}
}

pub fn square_clicked(
	mut commands: Commands,
	mut board_render_state: ResMut<resources::board_renderstate::BoardRenderState>,
	board_piece_state: Res<resources::board_piecestate::BoardPieceState>,
	chess: Res<shakmaty::Chess>,
	mut event_reader: EventReader<plugins::hitarea::InteractionEvent>,
) {
	for event in event_reader.iter() {
		let square = board_render_state.get_square_by_entity(event.entity).unwrap();
		match event.state {
			ButtonState::Pressed => {
				println!("Clicked {:?} {:?}", square.x, square.y);
				if board_render_state.is_selected_square(square.x, square.y) {
					// player clicked on square that was already selected - deselect it	
					board_render_state.clear_selected_square(&mut commands);
				} else {
					let has_selected = board_render_state.has_selected_square();
					let piece = chess.board().piece_at(shakmaty::Square::from_coords(shakmaty::File::ALL[square.x as usize], shakmaty::Rank::ALL[square.y as usize])); //board_piece_state.get_piece(square.x, square.y);
					let mut enemy_occupied = false;
					let mut friendly_occupied = false;
					match piece {
						Some(board_piece) => {
							enemy_occupied = board_piece.color != shakmaty::Color::White;
							friendly_occupied = board_piece.color == shakmaty::Color::White;
						},
						_ => {},
					}
					if has_selected {
						if enemy_occupied {
							// capture enemy piece if occupied by other side
						} else if friendly_occupied {
							// select the new piece
							board_render_state.set_selected_square(&mut commands, square.x, square.y);
						} else {
							// move selected piece to new empty position
						}
					} else {
						if enemy_occupied {
							// TODO: display which pieces are under threat from this enemy piece
						} else if friendly_occupied {
							board_render_state.set_selected_square(&mut commands, square.x, square.y);
						}
					}
				}
			},
			ButtonState::Released => {
				println!("Released {:?} {:?}", square.x, square.y);
			},
			_ => {},
		}
	}
}

pub fn on_exit(
	mut commands: Commands,
//	mut query: Query<(&Square, Entity)>,
) {
	// for (_square, entity) in query.iter_mut() {
	// 	commands.entity(entity).despawn_recursive();
	// }
}


pub fn escape_key(
	keys: Res<Input<KeyCode>>,
	mut state: ResMut<State<consts::GameState>>,
) {
	if keys.just_pressed(KeyCode::Escape) {
		state.set(consts::GameState::Menu).unwrap();
	}
}


pub fn on_piece_moveto(
	mut commands: Commands,
	// mut board_render_state: ResMut<BoardRenderState>,
	// mut board_state: ResMut<BoardState>,
	// mut square_entities: ResMut<HashMap<SquarePosition, Entity>>,
	// piece_map: ResMut<consts::PieceMap>,
	// mut moveto_query: Query<(Entity, &MoveTo, &BoardPiece, &mut Transform), (Changed<MoveTo>, With<BoardPiece>)>,
) {
	// for (entity, moveto, board_piece, mut transform) in moveto_query.iter_mut() {
	// 	println!("Moving: {0}x{1} -> {2}x{3}", moveto.from.x, moveto.from.y, moveto.to.x, moveto.to.y);
	// 	// Update the board state when the square_entities resource is changed
	// 	board_state.update_piece_position(&moveto);
	// 	let pos = consts::get_square_position(moveto.to.x, moveto.to.y);
	// 	let square_render = &board_render_state.squares[moveto.to.x as usize][moveto.to.y as usize];
	// 	transform.translation.x = pos.0;
	// 	transform.translation.y = pos.1;
	// }
}
