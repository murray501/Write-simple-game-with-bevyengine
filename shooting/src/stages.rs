use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Start,
    InGame,
    Finish,
}

struct ButtonData {
    button_entity: Entity,
}

fn enter_start(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    add_button(commands, asset_server, materials, "Start", 150.0);
}

fn enter_finish(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    add_button(commands, asset_server, materials, "GameOver", 200.0);
}

fn add_button(mut commands: Commands,asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>,
            buttontext: &str, buttonwidth: f32) {

    let button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(buttonwidth), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    buttontext,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .id();
    commands.insert_resource(ButtonData { button_entity });    
}

fn start(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<&Interaction, With<Button>>,
) {
    for interaction in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                state.set(AppState::InGame).unwrap();
            }
            _ => {}
        }
    }
}

fn cleanup(mut commands: Commands, button_data: Res<ButtonData>) {
    commands
        .entity(button_data.button_entity)
        .despawn_recursive();
}

pub fn add_other_states(appbuilder: &mut AppBuilder) -> &mut AppBuilder {
    appbuilder
        .add_state(AppState::Start)
        .add_system_set(SystemSet::on_enter(AppState::Start).with_system(enter_start.system()))
        .add_system_set(SystemSet::on_update(AppState::Start).with_system(start.system()))
        .add_system_set(SystemSet::on_exit(AppState::Start).with_system(cleanup.system()))
        .add_system_set(SystemSet::on_enter(AppState::Finish).with_system(enter_finish.system()))
}