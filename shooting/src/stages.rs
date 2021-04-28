use bevy::prelude::*;
use crate::{Enemies, Cannon, scoreboard_reset, Params};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Start,
    InGame,
    Finish,
}

fn enter_start(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
   
    //params
    let window = windows.get_primary().unwrap();
    commands.insert_resource(
        Params {
            bounds: Vec2::new(window.width(), window.height()),
            cannon: Vec2::new(40.0, 40.0),
            wall: 20.0,
            ball: Vec2::new(10.0, 10.0),
        }
    );  

    add_button(commands, asset_server, materials, "Start", 150.0); 
}

fn enter_finish(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
){
    add_button(commands, asset_server, materials, "GameOver", 200.0);
}

fn add_button(mut commands: Commands,asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>,
            buttontext: &str, buttonwidth: f32) {
    commands
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
        });  
}

fn button(
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

pub fn cleanup(mut commands: Commands, mut query: Query<Entity, With<Button>>) {
    let button_entity = query.single_mut().unwrap();
    commands
        .entity(button_entity)
        .despawn_recursive();
}

pub fn add_other_states(appbuilder: &mut AppBuilder) -> &mut AppBuilder {
    appbuilder
        .add_system_set(SystemSet::on_enter(AppState::Start).with_system(enter_start.system()))
        .add_system_set(SystemSet::on_update(AppState::Start).with_system(button.system()))
        .add_system_set(SystemSet::on_enter(AppState::Finish).with_system(enter_finish.system()))  
        .add_system_set(SystemSet::on_update(AppState::Finish).with_system(button.system()))
        .add_system_set(SystemSet::on_exit(AppState::Finish)
            .with_system(cleanup.system())
            .with_system(Enemies::cleanup.system())
            .with_system(Cannon::reset.system())
            .with_system(scoreboard_reset.system())
        )
}