use crate::consts;
use crate::components;
use crate::resources;
use crate::plugins;
use bevy::input::*;
use bevy::prelude::*;
use shakmaty::{Position, Move};

pub fn on_startup(
	mut commands: Commands,
	mut board_render_state: ResMut<resources::board_renderstate::BoardRenderState>,
	mut board_piece_state: ResMut<resources::board_piecestate::BoardPieceState>,
	chess: Res<shakmaty::Chess>,
	asset_server: Res<AssetServer>,
	texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	board_render_state.init(&mut commands);
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
		sprite.color = board_render_state.get_square_color(square.square(), selected_state.clone(), possible_move_state.clone());
	}
}

pub fn change_square_colour_on_possible_move_change(
	mut selected_query: Query<(&components::board::SquarePossibleMoveState, &components::board::SquarePosition, &components::board::SquareSelectedState, &mut Sprite), (Changed<components::board::SquarePossibleMoveState>, With<components::board::SquarePosition>, With<components::board::SquareSelectedState>, With<Sprite>)>,
	board_render_state: Res<resources::board_renderstate::BoardRenderState>,
) {
	for (possible_move_state, square, selected_state, mut sprite) in selected_query.iter_mut() {
		sprite.color = board_render_state.get_square_color(square.square(), selected_state.clone(), possible_move_state.clone());
	}
}

pub fn show_possible_moves_on_state_change(
	mut commands: Commands,
	chess: Res<shakmaty::Chess>,
	mut board_render_state: ResMut<resources::board_renderstate::BoardRenderState>,
	mut event_reader: EventReader<components::board::SquareSelectedEvent>,
) {
	for event in event_reader.iter() {
		match event.square {
			Some(square) => {
				let suggested_move = resources::cpu_player::get_best_move(&chess, 2);
				println!("best move: {:?}", suggested_move);
				board_render_state.clear_possible_moves(&mut commands);
				let legal_moves = chess.legal_moves();
				let mut moves: Vec<shakmaty::Square> = vec!();
				for m in &legal_moves {
					// change colour of potential move squares
					match m.from() {
						Some(from) => {
							if from == square {
								moves.push(m.to());
							}
						},
						_ => {},
					}
				}
				board_render_state.set_possible_moves(&mut commands, &moves);
				board_render_state.set_recommended_move(&mut commands, suggested_move.0.from().unwrap());
				board_render_state.set_recommended_move(&mut commands, suggested_move.0.to());
			},
			_ => {
				board_render_state.clear_possible_moves(&mut commands);
			},
		}
	}
}

pub fn square_clicked(
	mut commands: Commands,
	mut board_render_state: ResMut<resources::board_renderstate::BoardRenderState>,
	mut board_piece_state: ResMut<resources::board_piecestate::BoardPieceState>,
	mut chess: ResMut<shakmaty::Chess>,
	mut event_reader: EventReader<plugins::hitarea::InteractionEvent>,
	mut event_writer: EventWriter<components::board::SquareSelectedEvent>,
) {
	for event in event_reader.iter() {
		let square = board_render_state.get_square_by_entity(event.entity).unwrap();
		match event.state {
			ButtonState::Pressed => {
				println!("Clicked {:?}", square);
				if board_render_state.is_selected_square(square) {
					// player clicked on square that was already selected - deselect it	
					board_render_state.clear_selected_square(&mut commands, &mut event_writer);
				} else {
					let has_selected = board_render_state.has_selected_square();
					let piece = chess.board().piece_at(square);
					let mut enemy_occupied = false;
					let mut friendly_occupied = false;
					let turn = chess.turn();
					match piece {
						Some(board_piece) => {
							enemy_occupied = board_piece.color != turn;
							friendly_occupied = board_piece.color == turn;
						},
						_ => {},
					}
					if has_selected {
						if enemy_occupied {
							// capture enemy piece if occupied by other side
						} else if friendly_occupied {
							// select the new piece
							board_render_state.set_selected_square(&mut commands, square, &mut event_writer);
						} else {
							// move selected piece to new empty position if it's an allowed move
							let selected_square = board_render_state.get_selected_square().unwrap();
							let selected_piece = chess.board().piece_at(selected_square).unwrap();
							let mut candidates = chess.legal_moves();
							candidates.retain(|m| {
								match *m {
									Move::Normal { role: r, to: t, from: f, .. } => t == square && r == selected_piece.role && f == selected_square,
									Move::Put { role: r, to: t } => false,
									Move::EnPassant { to: t, .. } => /*role == Role::Pawn && t == to*/false,
									Move::Castle { .. } => false,
								}
							});
							if candidates.len() > 0 {
								chess.play_unchecked(&candidates[0]);
								board_render_state.clear_selected_square(&mut commands, &mut event_writer);
								board_piece_state.move_piece(&mut commands, &selected_piece, selected_square, square);
							}
						}
					} else {
						if enemy_occupied {
							// TODO: display which pieces are under threat from this enemy piece
						} else if friendly_occupied {
							board_render_state.set_selected_square(&mut commands, square, &mut event_writer);
						}
					}
				}
			},
			ButtonState::Released => {
				println!("Released {:?}", square);
			},
		}
	}
}

pub fn on_exit(
	mut _commands: Commands,
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
	mut board_render_state: ResMut<resources::board_renderstate::BoardRenderState>,
	mut board_piece_state: ResMut<resources::board_piecestate::BoardPieceState>,
	mut moveto_query: Query<(Entity, &components::pieces::MoveTo, &components::pieces::BoardPiece, &mut Transform), (Changed<components::pieces::MoveTo>, With<components::pieces::BoardPiece>)>,
) {
	for (entity, moveto, board_piece, mut transform) in moveto_query.iter_mut() {
		println!("Moving: {:?} -> {:?}", moveto.from, moveto.to);
		let pos = consts::get_square_position(moveto.to);
		transform.translation.x = pos.0;
		transform.translation.y = pos.1;
		commands.entity(entity).remove::<components::pieces::MoveTo>();
	}
}
