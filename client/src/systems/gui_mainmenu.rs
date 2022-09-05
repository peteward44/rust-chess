use crate::consts;
use crate::components;
use bevy::prelude::*;

// Displays user menu

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn add_button(
	commands: &mut ChildBuilder,
	asset_server: &Res<AssetServer>,
	name: &str,
) {
	commands
		.spawn_bundle(ButtonBundle {
			style: Style {
				size: Size::new(Val::Px(250.0), Val::Px(65.0)),
				// center button
				margin: UiRect::all(Val::Auto),
				// horizontally center child text
				justify_content: JustifyContent::Center,
				// vertically center child text
				align_items: AlignItems::Center,
				..Default::default()
			},
			color: NORMAL_BUTTON.into(),
			..Default::default()
		})
		.with_children(|parent| {
			parent.spawn_bundle(TextBundle {
				text: Text::from_section(
					name,
					TextStyle {
						font: asset_server.get_handle("fonts/primary/FiraSans-Bold.ttf"),
						font_size: 40.0,
						color: Color::rgb(0.9, 0.9, 0.9),
					},
				),
				..Default::default()
			});
		});
}


pub fn on_enter(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	// root node
	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..Default::default()
			},
			color: Color::rgb(0.1, 0.1, 0.20).into(),
			..Default::default()
		})
		.insert(components::gui::NeedsDespawning)
		.with_children(|grandparent| {
			// node holding button column
			grandparent
				.spawn_bundle(NodeBundle {
					style: Style {
						size: Size::new(Val::Percent(30.0), Val::Percent(50.0)),
						flex_direction: FlexDirection::ColumnReverse,
						justify_content: JustifyContent::Center,
						align_items: AlignItems::Center,
						..Default::default()
					},
					color: Color::NONE.into(),
					..Default::default()
				})
				.with_children(|mut parent| {
					add_button(&mut parent, &asset_server, "Resume Game");
					add_button(&mut parent, &asset_server, "New Game");
					add_button(&mut parent, &asset_server, "Quit");
				});
		});
}


pub fn on_exit(
	mut commands: Commands,
	mut query: Query<(&components::gui::NeedsDespawning, Entity)>,
) {
	for (_, entity) in query.iter_mut() {
		commands.entity(entity).despawn_recursive();
	}
}


pub fn button_system(
	mut interaction_query: Query<(&Interaction, &mut UiColor, &Children), (Changed<Interaction>, With<Button>)>,
	mut text_query: Query<&mut Text>,
	mut state: ResMut<State<consts::GameState>>,
) {
	for (interaction, mut color, children) in interaction_query.iter_mut() {
		let text = text_query.get_mut(children[0]).unwrap();
		match *interaction {
			Interaction::Clicked => {
				// text.sections[0].value = "Press".to_string();
				*color = PRESSED_BUTTON.into();
				match text.sections[0].value.as_str() {
					"Resume Game" => {}
					"New Game" => {
						state.set(consts::GameState::Playing).unwrap();
						return;
					}
					"Quit" => {
						state.set(consts::GameState::Quit).unwrap();
						return;
					}
					_ => {}
				}
			}
			Interaction::Hovered => {
				*color = HOVERED_BUTTON.into();
				// text.sections[0].value = "Hover".to_string();
			}
			Interaction::None => {
				*color = NORMAL_BUTTON.into();
				// text.sections[0].value = "Button".to_string();
			}
		}
	}
}
