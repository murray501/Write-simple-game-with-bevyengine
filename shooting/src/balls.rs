use crate::{Collider, Params, TIME_STEP, Cannon};
use bevy::prelude::*;

pub struct BallTimer;

pub struct Balls;

pub struct Ball {
    speed: Vec2,
}

impl Balls {
    pub fn spawner(mut commands: Commands, mouse_input: Res<Input<MouseButton>>, mut materials: ResMut<Assets<ColorMaterial>>, 
        query: Query<(&Cannon, &Sprite, &Transform)>, params: Res<Params>, 
        mut query_timer: Query<&mut Timer, With<BallTimer>>, time: Res<Time>, windows: Res<Windows>)
    {
        let mut can_shoot = true;

        if let Ok(mut timer) = query_timer.single_mut() {
            if !timer.tick(time.delta()).finished() {
                can_shoot = false;
            }
        }

        let (_cannon, sprite,transform) = query.single().unwrap();
        let cannon_position = Vec2::new(transform.translation.x + sprite.size.x * 0.5, transform.translation.y);
        
        if mouse_input.just_pressed(MouseButton::Left) && can_shoot {
            let win = windows.get_primary().unwrap();
            let cursor_position = win.cursor_position().unwrap();
            let size = Vec2::new(win.width() as f32, win.height() as f32);
            let mouse_position = cursor_position - size * 0.5;
            let ball_speed = (mouse_position - cannon_position).normalize_or_zero() * 500.0;
            
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                transform: Transform::from_xyz(cannon_position.x, cannon_position.y, 2.0),
                sprite: Sprite::new(params.ball.clone()),
                ..Default::default()
            })
            .insert(Ball { speed: ball_speed })
            .insert(Collider::Ball);
            
            if let Ok(mut timer) = query_timer.single_mut() {
                timer.reset();
            } else {
                commands.spawn().insert(Timer::from_seconds(0.5, false))
                    .insert(BallTimer);
            }
        }
    }

    pub fn update(mut commands: Commands, mut query_balls: Query<(Entity, &Ball, &mut Transform)>, params: Res<Params>){
        let xbound = params.bounds.x / 2.0 - params.wall;
        let ybound = params.bounds.y / 2.0 - params.wall;
        for (entity, ball, mut transform) in query_balls.iter_mut() {
            transform.translation.x += ball.speed.x * TIME_STEP;
            transform.translation.y += ball.speed.y * TIME_STEP;
            
            if transform.translation.x >= xbound {
                commands.entity(entity).despawn();                
            } else if transform.translation.x <= -xbound {
                commands.entity(entity).despawn();
            } else if transform.translation.y >= ybound {
                commands.entity(entity).despawn();
            } else if transform.translation.y <= -ybound {
                commands.entity(entity).despawn();
            }
        }
    }
}