use crate::{Collider, Params, TIME_STEP, Cannon};
use bevy::prelude::*;

pub struct Balls {
    speed: f32,
}

impl Balls {
    pub fn spawner(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>, mut materials: ResMut<Assets<ColorMaterial>>, 
        query: Query<(&Cannon, &Transform)>, params: Res<Params>)
    {
        let (_cannon, transform) = query.single().unwrap();
        let pos = &transform.translation;
        
        if keyboard_input.pressed(KeyCode::Space){
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
                transform: Transform::from_xyz(pos.x, pos.y, 2.0),
                sprite: Sprite::new(params.ball.clone()),
                ..Default::default()
            })
            .insert(Balls { speed: 500.0 })
            .insert(Collider::Ball);
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