use bevy::prelude::*;
use crate::{TIME_STEP, Params};

#[derive(Clone)]
pub struct Particles {
    pub speed: f32,
    pub num_divide:usize,
    pub material: Handle<ColorMaterial>,
}

pub struct Particle {
    speed: Vec2,
}

impl Particles {
    pub fn spawn(
        commands: &mut Commands,
        pos: Vec2,
        me: Particles,
    ) {
        let unit = std::f32::consts::PI * 2.0 / me.num_divide as f32;

        for i in 0..me.num_divide {
            let angle = unit * i as f32;
            let speed_vec = Vec2::new(angle.cos(), angle.sin()) * me.speed;
            commands
                .spawn_bundle(SpriteBundle {
                    material: me.material.clone(),
                    transform: Transform::from_xyz(pos.x, pos.y, 1.0),
                    sprite: Sprite::new(Vec2::new(512.0, 512.0)),
                    ..Default::default()
                })
                .insert(Particle{ speed: speed_vec });
        }
    }

    pub fn update(mut commands: Commands, mut query: Query<(Entity, &Particle, &Sprite, &mut Transform)>){
        for (entity, particle, sprite, mut transform) in query.iter_mut() {
            transform.translation.x += particle.speed.x * TIME_STEP;
            transform.translation.y += particle.speed.y * TIME_STEP;
            transform.scale *= 0.9;
            let size = sprite.size.x * transform.scale.x;
            if size < 5.0 {
                commands.entity(entity).despawn();
            }
        }
    } 
}