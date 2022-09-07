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

pub struct SquareSelectedEvent {
    pub square: Option<shakmaty::Square>,
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum SquarePossibleMoveState {
	None,
	PossibleMove,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SquarePosition {
	index: usize,
}

impl SquarePosition {
    pub fn new(square: shakmaty::Square) -> Self {
        Self { index: square as usize }
    }

    pub fn square(&self) -> shakmaty::Square {
        shakmaty::Square::ALL[self.index] 
    }
}

impl Add for SquarePosition {
    type Output = SquarePosition;

    fn add(self, rhs: Self) -> Self::Output {
        Self{ index: self.index + rhs.index }
    }
}

impl Mul<i32> for SquarePosition {
    type Output = SquarePosition;

    fn mul(self, rhs: i32) -> Self::Output {
        Self{ index: self.index * rhs as usize }
    }
}

// #[derive(Component, Debug, Clone)]
// pub struct MoveTo {
// 	pub from: SquarePosition,
// 	pub to: SquarePosition,
// }


// // events
// pub struct PieceMoved;
