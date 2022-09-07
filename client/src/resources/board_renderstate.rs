use bevy::prelude::*;
use crate::consts;
use crate::components;
use crate::plugins::hitarea::SpritePickerBundle;

pub struct BoardRenderState {
	selected: Option<shakmaty::Square>,
	entity_id_map: [Option<Entity>; shakmaty::Square::ALL.len() as usize],
}


impl BoardRenderState {
	pub fn new() -> Self {
		let entity_id_map: [Option<Entity>; shakmaty::Square::ALL.len() as usize] = array_init::array_init(|_i: usize| None);

		BoardRenderState {
			selected: None,
			entity_id_map,
		}
	}

	pub fn init(
		&mut self,
		commands: &mut Commands,
	) {
		for square in shakmaty::Square::ALL {
			let pos = consts::get_square_position(square);
			let entity_id = commands
				.spawn_bundle(SpriteBundle {
					transform: Transform::from_translation(Vec3::new(pos.0, pos.1, 0.0)),
					sprite: Sprite {
						custom_size: Some(Vec2::new(consts::SQUARE_WIDTH, consts::SQUARE_HEIGHT)),
						color: self.get_default_square_color(square),
						..Default::default()
					},
					..Default::default()
				})
				.insert_bundle(SpritePickerBundle::default())
				.insert(components::board::SquarePosition::new(square))
				.insert(components::board::SquareSelectedState::None)
				.insert(components::board::SquarePossibleMoveState::None)
				.id();
			self.entity_id_map[square as usize] = Some(entity_id);
		}
	}

	fn get_default_square_color(
		&self,
		square: shakmaty::Square,
	) -> Color {
		let white = Color::rgb(consts::BOARD_COLOUR1.0, consts::BOARD_COLOUR1.1, consts::BOARD_COLOUR1.2);
		let black = Color::rgb(consts::BOARD_COLOUR2.0, consts::BOARD_COLOUR2.1, consts::BOARD_COLOUR2.2);
		if square.is_dark() {
			return black;
		}
		return white;
	}

	
	pub fn get_square_color(
		&self,
		square: shakmaty::Square,
		selected_state: components::board::SquareSelectedState,
		possible_move_state: components::board::SquarePossibleMoveState,
	) -> Color {
		match selected_state {
			components::board::SquareSelectedState::Selected => Color::rgb(1.0, 1.0, 1.0).into(),
			_ => {
				match possible_move_state {
					components::board::SquarePossibleMoveState::PossibleMove => Color::rgb(0.5, 0.5, 0.5).into(),
					_ => self.get_default_square_color(square),
				}
			},
		}
	}

	pub fn reset_all(
		&mut self,
		commands: &mut Commands,
	) {
		for square in shakmaty::Square::ALL {
			let entity_id = self.entity_id_map[square as usize].unwrap();
			commands.entity(entity_id)
				.insert(components::board::SquareSelectedState::None)
				.insert(components::board::SquarePossibleMoveState::None);
		}
	}

	pub fn get_square_by_entity(
		&self,
		entity: Entity,
	) -> Option<shakmaty::Square> {
		for square in shakmaty::Square::ALL {
			match self.entity_id_map[square as usize] {
				Some(rhs) => {
					if entity == rhs {
						return Some(square);
					}
				}
				_ => {},
			}
		}
		None
	}

	pub fn get_selected_square(
		&self,
	) -> Option<shakmaty::Square> {
		self.selected
	}

	pub fn is_selected_square(
		&self,
		square: shakmaty::Square,
	) -> bool {
		match self.selected {
			Some(selected_square) => selected_square == square,
			None => false,
		}
	}

	pub fn has_selected_square(
		&self
	) -> bool {
		match self.selected {
			Some(_selected_square) => true,
			None => false,
		}
	}

	pub fn set_selected_square(
		&mut self,
		commands: &mut Commands,
		square: shakmaty::Square,
		event_writer: &mut EventWriter<components::board::SquareSelectedEvent>,
	) {
		match self.selected {
			Some(square) => {
				commands.entity(self.entity_id_map[square as usize].unwrap()).insert(components::board::SquareSelectedState::None);
			},
			_ => {},
		};
		commands.entity(self.entity_id_map[square as usize].unwrap()).insert(components::board::SquareSelectedState::Selected);
		self.selected = Some(square);
		event_writer.send(components::board::SquareSelectedEvent{ square: self.selected });
	}

	pub fn clear_selected_square(
		&mut self,
		commands: &mut Commands,
		event_writer: &mut EventWriter<components::board::SquareSelectedEvent>,
	) {
		match self.selected {
			Some(square) => {
				commands.entity(self.entity_id_map[square as usize].unwrap()).insert(components::board::SquareSelectedState::None);
			},
			_ => {},
		};
		self.selected = None;
		event_writer.send(components::board::SquareSelectedEvent{ square: self.selected });
	}

	pub fn set_possible_moves(
		&mut self,
		commands: &mut Commands,
		squares: &Vec<shakmaty::Square>,
	) {
		self.clear_possible_moves(commands);
		for square in squares.iter() {
			commands.entity(self.entity_id_map[*square as usize].unwrap()).insert(components::board::SquarePossibleMoveState::PossibleMove);
		}
	}

	pub fn clear_possible_moves(
		&mut self,
		commands: &mut Commands,
	) {
		for square in shakmaty::Square::ALL {
			commands.entity(self.entity_id_map[square as usize].unwrap()).insert(components::board::SquarePossibleMoveState::None);
		}
	}
}
