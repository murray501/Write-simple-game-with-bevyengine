use bevy::{
    core::FixedTimestep,
    prelude::*,
};

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.7,0.7, 0.7)))
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(ball_movement_system.system())
        )
        .run();
}

struct Ball {
    velocity: Vec3,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>){
    //camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    //ball
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
        transform: Transform::from_xyz(0.0, -50.0, 1.0),
        sprite: Sprite::new(Vec2::new(30.0, 30.0)),
        ..Default::default()
    })
    .insert(Ball {
        velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
    });
}

fn ball_movement_system(mut ball_query: Query<(&Ball, &mut Transform)>){
    if let Ok((ball, mut transform)) = ball_query.single_mut() {
        transform.translation += ball.velocity * TIME_STEP;
    }
}