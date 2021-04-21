use crate::{Collider, Params, TIME_STEP};
use bevy::prelude::*;
pub struct Cannon {
    speed: f32,
}

impl Cannon {
    pub fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, params: Res<Params>)
    {
        let wall_thickness = params.wall;
        let size = params.cannon.clone();
        let bounds = &params.bounds;

        commands.spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(-bounds.x / 2.0 + wall_thickness / 2.0 + size.x, 0.0, 1.0),
            sprite: Sprite::new(size),
            ..Default::default()
        })
        .insert(Cannon { speed: 500.0 })
        .insert(Collider::Cannon);
    } 

    pub fn update(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Cannon, &mut Transform)>, params: Res<Params>) {
        let (it, mut transform) = query.single_mut().unwrap();
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Up){
            direction += 1.0;
        }      
        
        if keyboard_input.pressed(KeyCode::Down){
            direction -= 1.0;
        } 

        let translation = &mut transform.translation;
        translation.y += direction * it.speed * TIME_STEP;

        let max = params.bounds.y / 2.0 - params.cannon.y / 2.0 - params.wall / 2.0; 
        translation.y = translation.y.min(max).max(-max);
    }
}