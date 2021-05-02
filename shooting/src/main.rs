mod cannon;
mod balls;
mod enemies;
mod stages;
mod particle;
mod enemyship;

use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::collide,
};

use cannon::Cannon;
use balls::{Balls, Ball};
use enemies::{Enemies, EnemyTimer, Direction};
use enemyship::{EnemyShipTimer, EnemyShips, EnemyShotTimer};
use stages::{AppState, add_other_states, cleanup};
use particle::Particles;
use std::env;

pub struct MainTimer(Timer);

pub const TIME_STEP: f32 = 1.0 / 60.0;

pub struct Params {
    pub background: Vec2,
    pub bounds: Vec2,
    pub cannon: Vec2,
    pub wall: f32,
    pub ball: Vec2,
    pub spacejunk_img: Handle<Texture>,
    pub spacejunk: Vec2,
    pub enemyship_img: Handle<Texture>,
    pub enemyship: Vec2,
    pub ball_self_color: Handle<ColorMaterial>,
    pub ball_enemy_color: Handle<ColorMaterial>,
}

pub struct Scoreboard {
    score: usize,
    health: usize,
}
#[derive(Eq, PartialEq)]
pub enum Collider {
    Spacejunk,
    Enemyship,
    Enemyball,
    Selfball,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut width: f32 = 1280.0;
    let mut height: f32 = 720.0;

    if args.len() == 3 {
        if let Ok(w) = args[1].parse::<f32>() {
            if let Ok(h) = args[2].parse::<f32>() {
                width = w;
                height = h;
            }    
        }
    }  
        
    let mut appbuilder = App::build();
    appbuilder
        .insert_resource(WindowDescriptor {
            title: "Space Shooter".to_string(),
            width: width,
            height: height,
            vsync: true,
            resizable: false,
            ..Default::default() 
        })
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Start)
        .insert_resource(Scoreboard { score: 0, health: 5 })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
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
            .with_system(Cannon::setup.system())
        )
        .add_system_set(SystemSet::on_update(AppState::InGame)
            .with_system(Cannon::update.system())
            .with_system(Cannon::collision.system())
            .with_system(Balls::update.system())
            .with_system(Balls::collision.system())
            .with_system(Enemies::update.system())
            .with_system(Balls::spawner.system())
            .with_system(Enemies::spawner.system())
            .with_system(scoreboard_system.system())
            .with_system(timer_system.system())
            .with_system(Particles::update.system())
            .with_system(EnemyShips::spawner.system())
            .with_system(EnemyShips::update.system())
            .with_system(EnemyShips::shoot.system())
            .with_system(cleanup_boundaries.system())
        )
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>)
{    
    commands.spawn().insert(Timer::from_seconds(1.0, false))
        .insert(EnemyTimer);

    commands.spawn().insert(Timer::from_seconds(5.0, false))
        .insert(EnemyShipTimer);   
    
    commands.spawn().insert(Timer::from_seconds(1.0, true))
        .insert(EnemyShotTimer);      

    // particles
    commands.insert_resource(Particles {
        speed: 100.0,
        num_divide: 8,
        material:materials.add(asset_server.load("images/imgbin_explosion-sprite-png.png").into())
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
                    value: "".to_string(),
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
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                },
                TextSection {
                    value: "\nHealth: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
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
    text.sections[5].value = scoreboard.health.to_string();
}

pub fn scoreboard_reset(mut scoreboard: ResMut<Scoreboard>, mut timer: ResMut<MainTimer>) {
    scoreboard.score = 0;
    scoreboard.health = 3;
    timer.0.reset();
}

fn timer_system(time: Res<Time>, mut timer: ResMut<MainTimer>, mut state: ResMut<State<AppState>>) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        state.set(AppState::Finish).unwrap();
    }
}

pub fn cleanup_colliders(mut commands: Commands, mut query: Query<Entity, With<Collider>>){
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_boundaries(mut commands: Commands, 
        params: Res<Params>, mut query: Query<(Entity, &Sprite, &Transform), With<Collider>>)
{
    for (entity, sprite, transform) in query.iter() {
        let maxx = params.background.x * 0.5 + sprite.size.x;
        let maxy = params.background.y * 0.5 + sprite.size.y;
        let x = transform.translation.x;
        let y = transform.translation.y;

        if x < -maxx || x > maxx || y < -maxy || y > maxy {
            commands.entity(entity).despawn();
        }
    }
}

