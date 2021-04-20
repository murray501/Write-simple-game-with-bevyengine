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
        .insert_resource(Scoreboard { score: 0})
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
                .with_system(ball_movement_system.system())
                .with_system(paddle_movement_system.system())
        )
        .add_system(scoreboard_system.system())
        .run();
}

struct Paddle {
    speed: f32,
    direction: Vec2,
}
struct Ball {
    velocity: Vec3,
}

struct Scoreboard {
    score: usize,
}

enum Collider {
    Solid,
    Scorable,
    Paddle,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
) {
    //camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
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
    
    //paddle
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(120.0, 30.0)),
            ..Default::default()
        })
        .insert(Paddle { speed: 500.0, direction: Vec2::ZERO })
        .insert(Collider::Paddle);

    //scoreboard    
    setup_scoreboard(commands, materials, windows, asset_server);
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

fn setup_scoreboard(
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
) {
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

    //walls
    setup_walls(commands, materials, windows);
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
    mut scoreboard: ResMut<Scoreboard>,
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
                    scoreboard.score += 1;
                } 
                else {
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

fn paddle_movement_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Paddle, &mut Transform, & Sprite)>, windows: Res<Windows>){
    if let Ok((mut paddle, mut transform, sprite)) = query.single_mut() {
        let speed = paddle.speed.to_owned();
        let direction = &mut paddle.direction;
    
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;            
        } else if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        } else if keyboard_input.pressed(KeyCode::Up) {
            direction.y +=  1.0;
        } else if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }

        *direction= direction.normalize_or_zero();
        
        let translation = &mut transform.translation;

        translation.x += direction.x * speed * TIME_STEP;
        translation.y += direction.y * speed * TIME_STEP;
        
        let window = windows.get_primary().unwrap();
        let xmax = window.width() * 0.5 - sprite.size.x * 0.5;
        let ymax = window.height() * 0.5 - sprite.size.y * 0.5;

        translation.x = translation.x.min(xmax).max(-1.0 * xmax);
        translation.y = translation.y.min(ymax).max(-1.0 * ymax);
    }
}

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut().unwrap();
    text.sections[1].value = scoreboard.score.to_string();
}