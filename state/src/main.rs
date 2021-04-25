use bevy::prelude::*;
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Start,
    InGame,
    Finish,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Start)
        .add_system_set(SystemSet::on_enter(AppState::Start).with_system(setup_start.system()))
        .add_system_set(SystemSet::on_update(AppState::Start).with_system(start.system()))
        .add_system_set(SystemSet::on_exit(AppState::Start).with_system(cleanup_menu.system()))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_game.system()))
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(ingame.system()))
        .add_system_set(SystemSet::on_enter(AppState::Finish).with_system(setup_finish.system()))
        .run();
}

struct StartData {
    button_entity: Entity,
}

fn cleanup_menu(mut commands: Commands, start_data: Res<StartData>) {
    commands
        .entity(start_data.button_entity)
        .despawn_recursive();
}

fn setup_start(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    let button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
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
                    "Play",
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
    commands.insert_resource(StartData { button_entity });
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

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert(Timer::from_seconds(10.0, false));
    
    // scoreboard
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn ingame(mut query_text: Query<&mut Text>,  mut query_timer: Query<&mut Timer>, time: Res<Time>,
        mut state: ResMut<State<AppState>>) {
    
    let mut timer = query_timer.single_mut().unwrap();
    if timer.tick(time.delta()).just_finished() {
        state.set(AppState::Finish).unwrap();         
    }

    let mut text = query_text.single_mut().unwrap();
    text.sections[1].value = timer.elapsed_secs().to_string();
}

fn setup_finish(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(65.0)),
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
                    "Game Over",
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
    commands.insert_resource(StartData { button_entity });
}
