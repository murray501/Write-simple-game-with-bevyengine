mod cannon;
mod walls;
mod balls;

use bevy::{
    core::FixedTimestep,
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

use cannon::Cannon;
use walls::Walls;
use balls::Balls;

pub const TIME_STEP: f32 = 1.0 / 60.0;

pub enum Collider {
    Cannon,
    Wall,
    Ball,
}

pub struct Params {
    bounds: Vec2,
    cannon: Vec2,
    wall: f32,
    ball: Vec2,
}

impl Params {
    fn new() -> Self {
        Params {
            bounds: Vec2::new(900.0, 600.0),
            cannon: Vec2::new(30.0, 120.0),
            wall: 20.0,
            ball: Vec2::new(10.0, 10.0),
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Params::new())
        .add_startup_system(setup_camera.system())
        .add_startup_system(Walls::setup.system())
        .add_startup_system(Cannon::setup.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(Cannon::update.system())
                .with_system(Balls::update.system())
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.2))
                .with_system(Balls::spawner.system())
        )
        .run();
}

fn setup_camera(mut commands: Commands){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}