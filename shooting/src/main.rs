mod cannon;
mod walls;
mod balls;
mod enemies;
mod stages;
mod particle;

use bevy::{
    prelude::*,
    render::pass::ClearColor,
};

use cannon::Cannon;
use walls::Walls;
use balls::{Balls, Ball};
use enemies::{Enemies, EnemyTimer, Enemy};
use stages::{AppState, add_other_states, cleanup};
use particle::Particles;

pub struct MainTimer(Timer);

pub const TIME_STEP: f32 = 1.0 / 60.0;
pub enum Collider {
    Cannon,
    Wall,
    Ball,
    Enemy
}

pub struct Params {
    pub background: Vec2,
    pub bounds: Vec2,
    pub cannon: Vec2,
    pub wall: f32,
    pub ball: Vec2,
    pub spacejunk_img: Handle<Texture>,
    pub spacejunk: Vec2,
}

pub struct Scoreboard {
    score: usize,
}

fn main() {
    let mut appbuilder = App::build();

    appbuilder
        /* 
        .insert_resource(WindowDescriptor {
            title: "Space Shooter".to_string(),
            width: 2048.0 * 0.7,
            height: 1536.0 * 0.5,
            vsync: true,
            resizable: false,
            ..Default::default() 
        })
        */
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Start)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(MainTimer(Timer::from_seconds(180.0, false)));
    
    add_other_states(&mut appbuilder);    
    add_game_state(&mut appbuilder);
    
    appbuilder.run();    
}

fn add_game_state(appbuilder: &mut AppBuilder) -> &mut AppBuilder {
    appbuilder
        .add_system_set(SystemSet::on_exit(AppState::Start)
            .with_system(cleanup.system())
            .with_system(setup.system())
            //.with_system(Walls::setup.system())
            .with_system(Cannon::setup.system())
        )
        .add_system_set(SystemSet::on_update(AppState::InGame)
            .with_system(Cannon::update.system())
            .with_system(Cannon::collision.system())
            .with_system(Balls::update.system())
            .with_system(Enemies::update.system())
            .with_system(Enemies::collision.system())
            .with_system(Balls::spawner.system())
            .with_system(Enemies::spawner.system())
            .with_system(scoreboard_system.system())
            .with_system(timer_system.system())
            .with_system(Particles::update.system()))
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>)
{    
    commands.spawn().insert(Timer::from_seconds(1.0, false))
        .insert(EnemyTimer);

    // particles
    commands.insert_resource(Particles {
        speed: 100.0,
        num_divide: 8,
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into())
    });   
        
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
                TextSection {
                    value: "\nTime: ".to_string(),
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
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }); 
}

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>, timer: Res<MainTimer>) {
    let mut text = query.single_mut().unwrap();
    text.sections[1].value = scoreboard.score.to_string();
    text.sections[3].value = (180.0 - timer.0.elapsed_secs().trunc()).to_string();
}

pub fn scoreboard_reset(mut scoreboard: ResMut<Scoreboard>, mut timer: ResMut<MainTimer>) {
    scoreboard.score = 0;
    timer.0.reset();
}

fn timer_system(time: Res<Time>, mut timer: ResMut<MainTimer>, mut state: ResMut<State<AppState>>) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        state.set(AppState::Finish).unwrap();
    }
}