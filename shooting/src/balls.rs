use crate::{Params, TIME_STEP, Cannon, Direction};
use bevy::prelude::*;

pub struct BallTimer;

pub struct Balls;

pub struct Ball {
    speed: Vec2,
}

impl Balls {
    pub fn spawn(commands: &mut Commands, pos: Vec2, direction: Vec2, params: &Res<Params>) {
        let material = params.ball_enemy_color.clone();
        commands.spawn_bundle(SpriteBundle {
            material: material,
            transform: Transform::from_xyz(pos.x, pos.y, 2.0),
            sprite: Sprite::new(params.ball.clone()),
            ..Default::default()
        })
        .insert(Ball { speed: direction * 500.0 });
    }

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

        let (cannon, sprite,transform) = query.single().unwrap();
        let cannon_position = 
            if cannon.direction == Direction::Right {
                Vec2::new(transform.translation.x + sprite.size.x * 0.5, transform.translation.y)
            } else {
                Vec2::new(transform.translation.x - sprite.size.x * 0.5, transform.translation.y)
            };
        
        if mouse_input.just_pressed(MouseButton::Left) && can_shoot {
            let win = windows.get_primary().unwrap();
            let cursor_position = win.cursor_position().unwrap();
            let size = Vec2::new(win.width() as f32, win.height() as f32);
            let mouse_position = cursor_position - size * 0.5;
            /* 
            if cannon.direction == Direction::Right && mouse_position.x < 0.0 {
                return;
            } 
            if cannon.direction == Direction::Left && mouse_position.x > 0.0 {
                return;
            }
            */
            let ball_speed = mouse_position.normalize_or_zero() * 500.0;
            
            commands.spawn_bundle(SpriteBundle {
                material: params.ball_self_color.clone(),
                transform: Transform::from_xyz(cannon_position.x, cannon_position.y, 2.0),
                sprite: Sprite::new(params.ball.clone()),
                ..Default::default()
            })
            .insert(Ball { speed: ball_speed });
            
            if let Ok(mut timer) = query_timer.single_mut() {
                timer.reset();
            } else {
                commands.spawn().insert(Timer::from_seconds(0.5, false))
                    .insert(BallTimer);
            }
        }
    }

    pub fn update(mut commands: Commands, mut query_balls: Query<(Entity, &Ball, &mut Transform)>, params: Res<Params>){
        let xbound = params.background.x / 2.0;
        let ybound = params.background.y / 2.0;
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