use bevy::prelude::*;
use std::ops::{Add, Mul};

#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
pub struct Square {
	color: Color,
}

// modifiers
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum SquareSelectedState {
	None,
	Selected,
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum SquarePossibleMoveState {
	None,
	PossibleMove,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SquarePosition {
	pub x: i32,
	pub y: i32,
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

// #[derive(Component, Debug, Clone)]
// pub struct MoveTo {
// 	pub from: SquarePosition,
// 	pub to: SquarePosition,
// }


// // events
// pub struct PieceMoved;
