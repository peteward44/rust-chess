use super::consts;
use bevy::prelude::*;

// Displays user menu

struct ButtonMaterials {
	normal: Handle<ColorMaterial>,
	hovered: Handle<ColorMaterial>,
	pressed: Handle<ColorMaterial>,
}

struct NeedsDespawning;


impl FromWorld for ButtonMaterials {
	fn from_world(world: &mut World) -> Self {
		let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
		ButtonMaterials {
			normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
			hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
			pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
		}
	}
}


fn add_button(
	commands: &mut ChildBuilder,
	asset_server: &Res<AssetServer>,
	button_materials: &Res<ButtonMaterials>,
	name: &str,
) {
	commands
		.spawn_bundle(ButtonBundle {
			style: Style {
				size: Size::new(Val::Px(250.0), Val::Px(65.0)),
				// center button
				margin: Rect::all(Val::Auto),
				// horizontally center child text
				justify_content: JustifyContent::Center,
				// vertically center child text
				align_items: AlignItems::Center,
				..Default::default()
			},
			material: button_materials.normal.clone(),
			..Default::default()
		})
		.with_children(|parent| {
			parent.spawn_bundle(TextBundle {
				text: Text::with_section(
					name,
					TextStyle {
						font: asset_server.get_handle("fonts/primary/FiraSans-Bold.ttf"),
						font_size: 40.0,
						color: Color::rgb(0.9, 0.9, 0.9),
					},
					Default::default(),
				),
				..Default::default()
			});
		});
}


fn on_enter(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>,
	button_materials: Res<ButtonMaterials>,
) {
	// camera
	commands.spawn_bundle(UiCameraBundle::default()).insert(NeedsDespawning);
	// root node
	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..Default::default()
			},
			material: materials.add(Color::rgb(0.1, 0.1, 0.20).into()),
			..Default::default()
		})
		.insert(NeedsDespawning)
		.with_children(|grandparent| {
			// node holding button column
			grandparent
				.spawn_bundle(NodeBundle {
					style: Style {
						size: Size::new(Val::Percent(30.0), Val::Percent(50.0)),
						//justify_content: JustifyContent::SpaceBetween,
						flex_direction: FlexDirection::ColumnReverse,
						justify_content: JustifyContent::Center,
						align_items: AlignItems::Center,
						..Default::default()
					},
					material: materials.add(Color::NONE.into()),
					..Default::default()
				})
				.with_children(|mut parent| {
					add_button(&mut parent, &asset_server, &button_materials, "Resume Game");
					add_button(&mut parent, &asset_server, &button_materials, "New Game");
					add_button(&mut parent, &asset_server, &button_materials, "Quit");
				});
		});
}


fn on_exit(
	mut commands: Commands,
	mut query: Query<(&NeedsDespawning, Entity)>,
) {
	for (_, entity) in query.iter_mut() {
		commands.entity(entity).despawn_recursive();
	}
}


fn button_system(
	button_materials: Res<ButtonMaterials>,
	mut interaction_query: Query<(&Interaction, &mut Handle<ColorMaterial>, &Children), (Changed<Interaction>, With<Button>)>,
	mut text_query: Query<&mut Text>,
	mut state: ResMut<State<consts::GameState>>,
) {
	for (interaction, mut material, children) in interaction_query.iter_mut() {
		let text = text_query.get_mut(children[0]).unwrap();
		match *interaction {
			Interaction::Clicked => {
				// text.sections[0].value = "Press".to_string();
				*material = button_materials.pressed.clone();
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
				// text.sections[0].value = "Hover".to_string();
				*material = button_materials.hovered.clone();
			}
			Interaction::None => {
				// text.sections[0].value = "Button".to_string();
				*material = button_materials.normal.clone();
			}
		}
	}
}


// Plugin
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(
		&self,
		app: &mut AppBuilder,
	) {
		app.init_resource::<ButtonMaterials>()
			.add_system(button_system.system())
			.add_system_set(SystemSet::on_enter(consts::GameState::Menu).with_system(on_enter.system()))
			.add_system_set(SystemSet::on_exit(consts::GameState::Menu).with_system(on_exit.system()));
	}
}
