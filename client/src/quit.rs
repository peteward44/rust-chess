use super::consts;
use bevy::prelude::*;

// Display splash screen on quit
struct DelayTimer(Timer);

fn on_enter(// mut commands: Commands,
	// mut materials: ResMut<Assets<ColorMaterial>>,
	// asset_server: Res<AssetServer>,
) {
}


fn on_update(
	time: Res<Time>,
	mut timer: ResMut<DelayTimer>,
	mut app_exit_events: EventWriter<bevy::app::AppExit>,
) {
	if timer.0.tick(time.delta()).just_finished() {
		app_exit_events.send(bevy::app::AppExit);
	}
}


// Plugin
pub struct QuitPlugin;

impl Plugin for QuitPlugin {
	fn build(
		&self,
		app: &mut AppBuilder,
	) {
		app.insert_resource(DelayTimer(Timer::from_seconds(2.0, true)))
			.add_system_set(
				SystemSet::on_enter(consts::GameState::Quit).with_system(on_enter.system()),
			)
			.add_system_set(
				SystemSet::on_update(consts::GameState::Quit).with_system(on_update.system()),
			);
	}
}
