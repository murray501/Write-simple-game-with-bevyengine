mod cannon;
mod walls;
mod balls;
mod enemies;

use bevy::{
    core::FixedTimestep,
    prelude::*,
    render::pass::ClearColor,
};

use cannon::Cannon;
use walls::Walls;
use balls::{Balls, Ball};
use enemies::{Enemies, EnemyTimer};

pub const TIME_STEP: f32 = 1.0 / 60.0;

pub enum Collider {
    Cannon,
    Wall,
    Ball,
    Enemy
}

pub struct Params {
    bounds: Vec2,
    cannon: Vec2,
    wall: f32,
    ball: Vec2,
}

pub struct Scoreboard {
    score: usize,
}

impl Params {
    fn new() -> Self {
        Params {
            bounds: Vec2::new(900.0, 600.0),
            cannon: Vec2::new(40.0, 40.0),
            wall: 20.0,
            ball: Vec2::new(10.0, 10.0),
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Params::new())
        .add_startup_system(setup.system())
        .add_startup_system(Walls::setup.system())
        .add_startup_system(Cannon::setup.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(Cannon::update.system())
                .with_system(Balls::update.system())
                .with_system(Enemies::update.system())
                .with_system(Enemies::collision.system())
        )
        .add_system(Balls::spawner.system())
        .add_system(Enemies::spawner.system())
        .add_system(scoreboard_system.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn().insert(Timer::from_seconds(1.0, false))
        .insert(EnemyTimer);

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

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut().unwrap();
    text.sections[1].value = scoreboard.score.to_string();
}