use bevy::prelude::*;
use crate::consts;
use crate::components;
use crate::plugins::hitarea::SpritePickerBundle;

pub struct BoardRenderState {
	selected: Option<components::board::SquarePosition>,
	entity_id_map: [[Option<Entity>; consts::BOARD_WIDTH as usize]; consts::BOARD_HEIGHT as usize],
}


impl BoardRenderState {
	pub fn new() -> Self {
		let entity_id_map: [[Option<Entity>; consts::BOARD_WIDTH as usize]; consts::BOARD_HEIGHT as usize] = array_init::array_init(|_x: usize| {
			array_init::array_init(|_y: usize| None)
		});

		BoardRenderState {
			selected: None,
			entity_id_map,
		}
	}

	pub fn init(
		&mut self,
		mut commands: &mut Commands,
	) {
		for y in 0..consts::BOARD_WIDTH {
			for x in 0..consts::BOARD_HEIGHT {
				let pos = consts::get_square_position(x, y);
				let entity_id = commands
					.spawn_bundle(SpriteBundle {
						transform: Transform::from_translation(Vec3::new(pos.0, pos.1, 0.0)),
						sprite: Sprite {
							custom_size: Some(Vec2::new(consts::SQUARE_WIDTH, consts::SQUARE_HEIGHT)),
							color: self.get_default_square_color(x, y),
							..Default::default()
						},
						..Default::default()
					})
					.insert_bundle(SpritePickerBundle::default())
					.insert(components::board::SquarePosition { x: x, y: y })
					.insert(components::board::SquareSelectedState::None)
					.insert(components::board::SquarePossibleMoveState::None)
					.id();
				self.entity_id_map[x as usize][y as usize] = Some(entity_id);
			}
		}
	}

	fn get_default_square_color(
		&self,
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

	
	pub fn get_square_color(
		&self,
		x: i32,
		y: i32,
		selected_state: components::board::SquareSelectedState,
		possible_move_state: components::board::SquarePossibleMoveState,
	) -> Color {
		match selected_state {
			components::board::SquareSelectedState::Selected => Color::rgb(1.0, 1.0, 1.0).into(),
			_ => {
				match possible_move_state {
					components::board::SquarePossibleMoveState::PossibleMove => Color::rgb(0.5, 0.5, 0.5).into(),
					_ => self.get_default_square_color(x, y),
				}
			},
		}
	}

	pub fn reset_all(
		&mut self,
		commands: &mut Commands,
	) {
		for y in 0..consts::BOARD_WIDTH {
			for x in 0..consts::BOARD_HEIGHT {
				let entity_id = self.entity_id_map[x as usize][y as usize].unwrap();
				commands.entity(entity_id)
					.insert(components::board::SquareSelectedState::None)
					.insert(components::board::SquarePossibleMoveState::None);
			}
		}
	}

	pub fn get_square_by_entity(
		&self,
		entity: Entity,
	) -> Option<components::board::SquarePosition> {
		for y in 0..consts::BOARD_WIDTH {
			for x in 0..consts::BOARD_HEIGHT {
				match self.entity_id_map[x as usize][y as usize] {
					Some(rhs) => {
						if entity == rhs {
							return Some(components::board::SquarePosition {
								x: x,
								y: y,
							});
						}
					}
					_ => {},
				}
			}
		}
		None
	}

	pub fn is_selected_square(
		&self,
		x: i32,
		y: i32,
	) -> bool {
		match self.selected {
			Some(selected_square) => selected_square.x == x && selected_square.y == y,
			None => false,
		}
	}

	pub fn has_selected_square(
		&self
	) -> bool {
		match self.selected {
			Some(selected_square) => true,
			None => false,
		}
	}

	pub fn set_selected_square(
		&mut self,
		commands: &mut Commands,
		x: i32,
		y: i32,
	) {
		self.clear_selected_square(commands);
		commands.entity(self.entity_id_map[x as usize][y as usize].unwrap()).insert(components::board::SquareSelectedState::Selected);
		self.selected = Some(components::board::SquarePosition{ x, y });
	}

	pub fn clear_selected_square(
		&mut self,
		commands: &mut Commands,
	) {
		match self.selected {
			Some(square) => {
				commands.entity(self.entity_id_map[square.x as usize][square.y as usize].unwrap()).insert(components::board::SquareSelectedState::None);
			},
			_ => {},
		};
		self.selected = None;
	}

	pub fn set_possible_move(
		&mut self,
		commands: &mut Commands,
		x: i32,
		y: i32,
	) {
		commands.entity(self.entity_id_map[x as usize][y as usize].unwrap()).insert(components::board::SquarePossibleMoveState::PossibleMove);
	}

	pub fn on_click_unoccupied_square(
		&mut self,
		commands: &mut Commands,
		x: i32,
		y: i32,
	) {
		// match self.selected {
		// 	Some(selected_square) => {
		// 		let piece = board_state.get_piece_by_position(selected_square.x, selected_square.y);
		// 		match piece {
		// 			Some(board_piece) => {
		// 				let possible_moves = Rules::get_piece_possible_moves(&board_state, selected_square.x, selected_square.y, &board_piece);
		// 				// see if clicked square is a possible move
		// 				println!("Move: {:?}", board_piece.piece);
		// 				let found = possible_moves.iter().find(|m| m.x == square.x && m.y == square.y);
		// 				match found {
		// 					Some(_found_move) => {
		// 						println!("Valid move: {:?}", board_piece.piece);
		// 						let ent = *piece_map.get(&board_piece.id).unwrap();
		// 						commands.entity(ent).insert(MoveTo{from: selected_square, to: *square});
		// 						commands.entity(entity).insert(SquareState::None);
		// 						board_render_state.selected = None;
		// 						for pmove in possible_moves.iter() {
		// 							let move_square = *square_entities.get(&SquarePosition{x: pmove.x, y: pmove.y}).unwrap();
		// 							commands.entity(move_square).insert(SquareState::None);
		// 						}
		// 						let home_square = *square_entities.get(&SquarePosition{x: selected_square.x, y: selected_square.y}).unwrap();
		// 						commands.entity(home_square).insert(SquareState::None);
		// 					}
		// 					_ => {}
		// 				}
		// 			}
		// 			_ => {}
		// 		}
		// 	}
		// 	None => {}
		// }
	}
}
