use crate::{Collider, Params, TIME_STEP, Cannon};
use bevy::prelude::*;

pub struct BallTimer;

pub struct Balls {
    speed: f32,
}

impl Balls {
    pub fn spawner(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>, mut materials: ResMut<Assets<ColorMaterial>>, 
        query: Query<(&Cannon, &Transform)>, params: Res<Params>, 
        mut query_timer: Query<&mut Timer, With<BallTimer>>, time: Res<Time>)
    {
        let mut can_shoot = true;

        if let Ok(mut timer) = query_timer.single_mut() {
            if !timer.tick(time.delta()).finished() {
                can_shoot = false;
            }
        }

        let (_cannon, transform) = query.single().unwrap();
        let pos = &transform.translation;
        
        if keyboard_input.pressed(KeyCode::Space) && can_shoot {
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                sprite: Sprite::new(params.ball.clone()),
                ..Default::default()
            })
            .insert(Balls { speed: 500.0 })
            .insert(Collider::Ball);
            
            if let Ok(mut timer) = query_timer.single_mut() {
                timer.reset();
            } else {
                commands.spawn().insert(Timer::from_seconds(0.5, false))
                    .insert(BallTimer);
            }
        }
    }

    pub fn update(mut commands: Commands, mut query: Query<(Entity, &Balls, &mut Transform)>, params: Res<Params>){
        let max = params.bounds.x / 2.0 - params.wall;
   
        for (entity, ball, mut transform) in query.iter_mut() {
            transform.translation.x += ball.speed * TIME_STEP;
            if transform.translation.x >= max {
                commands.entity(entity).despawn();                
            }
        }
    }
}