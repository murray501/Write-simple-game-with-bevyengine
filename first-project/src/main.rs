use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use rand::prelude::random;

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.0))
                .with_system(brick_spawner.system())
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(ball_collision_system.system())
                .with_system(ball_movement_system.system()),
        )
        .run();
}

struct Ball {
    velocity: Vec3,
}

enum Collider {
    Solid,
    Scorable,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    //camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    //ball
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, -50.0, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
    setup_walls(commands, materials, windows);
}

fn brick_spawner(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let width = window.width();
    let height = window.height();
    let x = random::<f32>() * width - width * 0.5;
    let y = random::<f32>() * height - height * 0.5;
    let material = materials.add(Color::rgb(1.0, 0.0, 1.0).into());
    commands
        .spawn_bundle(SpriteBundle {
            material: material.clone(),
            transform: Transform::from_xyz(x, y, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(Collider::Scorable);
}

fn setup_walls(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 20.0;
    let bounds = Vec2::new(window.width(), window.height());

    let transforms = vec![
        Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
        Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
        Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
        Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
    ];

    let sprites = vec![
        Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
        Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
    ];

    for (transform, sprite) in transforms.into_iter().zip(sprites.into_iter()) {
        commands
            .spawn_bundle(SpriteBundle {
                material: wall_material.clone(),
                transform: transform,
                sprite: sprite,
                ..Default::default()
            })
            .insert(Collider::Solid);
    }
}

fn ball_movement_system(mut ball_query: Query<(&Ball, &mut Transform)>) {
    if let Ok((ball, mut transform)) = ball_query.single_mut() {
        transform.translation += ball.velocity * TIME_STEP;
    }
}

fn ball_collision_system(
    mut commands: Commands,
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {
    if let Ok((mut ball, ball_transform, sprite)) = ball_query.single_mut() {
        let ball_size = sprite.size;
        let velocity = &mut ball.velocity;

        for (collider_entity, collider, transform, sprite) in collider_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                sprite.size,
            );
            if let Some(collision) = collision {
                if let Collider::Scorable = *collider {
                    commands.entity(collider_entity).despawn();
                } 
                else if let Collider::Solid = *collider {
                    match collision {
                        Collision::Left => {
                            if velocity.x > 0.0 {
                                velocity.x = -velocity.x;
                            }
                        }
                        Collision::Right => {
                            if velocity.x < 0.0 {
                                velocity.x = -velocity.x;
                            }
                        }
                        Collision::Top => {
                            if velocity.y < 0.0 {
                                velocity.y = -velocity.y;
                            }
                        }
                        Collision::Bottom => {
                            if velocity.y > 0.0 {
                                velocity.y = -velocity.y;
                            }
                        }
                    }
                }
            }
        }
    }
}
