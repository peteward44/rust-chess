use bevy::prelude::*;

#[derive(Resource)]
pub struct GuiQuit {
	pub timer: Timer
}

impl GuiQuit {
	pub fn new() -> Self {
		GuiQuit {
			timer: Timer::from_seconds(2.0, TimerMode::Once)
		}
	}
}
