use crate::{Collider, Params};
use bevy::prelude::*;

pub struct Walls;

impl Walls {
    pub fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, params: Res<Params>){
        let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
        let wall_thickness = params.wall;
        let bounds = &params.bounds;
    
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
                .insert(Collider::Wall);
        }       
    }
}