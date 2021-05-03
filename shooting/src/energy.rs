use bevy::prelude::*;
use rand::Rng;
use crate::{Collider, Params};
use bevy::sprite::collide_aabb::collide;
pub struct Energy;

impl Energy {
    pub fn setup(mut commands: Commands, params: Res<Params>) {
        let mut rng = rand::thread_rng();
        let bounds = params.background.to_owned() * 0.5 - params.energy.to_owned() * 0.5;
        for i in 0..params.num_of_energies {
            let x = rng.gen_range(-bounds.x..bounds.x);
            let y = rng.gen_range(-bounds.y..bounds.y);
            commands.spawn_bundle(SpriteBundle {
                material: params.energy_img.clone(),
                transform: Transform::from_xyz(x, y, 0.0),
                sprite: Sprite::new(params.energy.clone()),
                ..Default::default()
            }).insert(Collider::Energy);
        }
    }

    pub fn collision(mut commands: Commands, 
        mut colliders: Query<(Entity, &Sprite, &Transform, &Collider)> 
    ){
        let energies = colliders.iter()
            .filter(|(_,_,_,collider)| **collider == Collider::Energy);

        for (entity, sprite, transform, _) in energies {
            let enemyballs = colliders.iter()
            .filter(|(_,_,_,collider)| **collider == Collider::Enemyball);
            for (entity2, sprite2, transform2, _) in enemyballs {
                let collision = collide(
                    transform.translation,
                    sprite.size,
                    transform2.translation,
                    sprite2.size
                );
                if collision.is_some() {
                    commands.entity(entity2).despawn();
                }
            }
        }
    }
}