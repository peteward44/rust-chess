use crate::resources::gui_quit::*;
use bevy::prelude::*;

// Display splash screen on quit
pub fn on_enter(// mut commands: Commands,
	// mut materials: ResMut<Assets<ColorMaterial>>,
	// asset_server: Res<AssetServer>,
) {
}


pub fn on_update(
	time: Res<Time>,
	mut gui_quit: ResMut<GuiQuit>,
	mut app_exit_events: EventWriter<bevy::app::AppExit>,
) {
	if gui_quit.timer.tick(time.delta()).just_finished() {
		app_exit_events.send(bevy::app::AppExit);
	}
}
