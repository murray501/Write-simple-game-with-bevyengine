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
    let background_size = Vec2::new(2048.0, 1536.0) * 1.5;
    commands.insert_resource(
        Params {
            background: background_size.to_owned(),
            bounds: Vec2::new(window.width(), window.height()),
            cannon: Vec2::new(200.0, 120.0) * 0.4,
            wall: 20.0,
            ball: Vec2::new(15.0, 15.0),
            spacejunk_img: asset_server.load("images/space-junk.png"),
            spacejunk: Vec2::new(250.0, 198.0),
            enemyship_img: asset_server.load("images/enemy-ship.png"),
            enemyship: Vec2::new(192.0, 250.0) * 0.3,
            ball_self_color: materials.add(Color::rgb(1.,1.,0.).into()),
            ball_enemy_color: materials.add(Color::rgb(0.,1.,1.).into()),
        }
    );  

    // background
    let texture_handle = asset_server.load("images/space.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_handle.into()),
        sprite: Sprite::new(background_size),
        ..Default::default()
    });

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
            material: materials.add(Color::rgba(0.3, 0.3, 0.3, 0.5).into()),
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