use crate::consts;
use shakmaty;
use bevy::prelude::*;

// #[derive(Component, Debug, Clone)]
// pub struct MoveTo {
// 	pub from: SquarePosition,
// 	pub to: SquarePosition,
// }


// // events
// pub struct PieceMoved;

#[derive(Component, Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct BoardPiece {
	pub piece: consts::PieceType,
	pub is_white: bool,
	pub id: consts::PieceId,
}

#[derive(Component, Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct BoardPieceShakmaty {
	pub piece: shakmaty::Piece,
}
