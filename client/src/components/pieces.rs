use shakmaty;
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct MoveTo {
	pub from: shakmaty::Square,
	pub to: shakmaty::Square,
}


#[derive(Component, Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct BoardPiece {
	pub piece: shakmaty::Piece,
	pub square: shakmaty::Square,
}
