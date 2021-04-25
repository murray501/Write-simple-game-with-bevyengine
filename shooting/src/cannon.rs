use crate::{Collider, Params, TIME_STEP, Enemy, AppState};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
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
            transform: Transform::from_xyz(-bounds.x * 0.5 + wall_thickness * 0.5 + size.x * 0.5, 0.0, 3.0),
            sprite: Sprite::new(size),
            ..Default::default()
        })
        .insert(Cannon { speed: 500.0 })
        .insert(Collider::Cannon);
    } 

    pub fn collision(mut commands: Commands, mut enemies: Query<(&Sprite, &Transform), With<Enemy>>,
        mut self_query: Query<(&Sprite, &Transform), With<Cannon>>, mut state: ResMut<State<AppState>>)
    {
        let (self_sprite, self_transform) = self_query.single_mut().unwrap();
    
        for (sprite, transform) in enemies.iter() {
            let collision = collide(
                self_transform.translation,
                self_sprite.size,
                transform.translation,
                sprite.size
            );
            if collision.is_some() {
                state.set(AppState::Finish).unwrap();
            }
        }
    }
    
    pub fn update(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Cannon, &mut Transform)>, params: Res<Params>) {
        let (it, mut transform) = query.single_mut().unwrap();
        let mut direction = Vec2::new(0.0, 0.0);
        if keyboard_input.pressed(KeyCode::Up){
            direction.y += 1.0;
        }      
        
        if keyboard_input.pressed(KeyCode::Down){
            direction.y -= 1.0;
        } 

        if keyboard_input.pressed(KeyCode::Left){
            direction.x -= 1.0;
        } 

        if keyboard_input.pressed(KeyCode::Right){
            direction.x += 1.0;
        } 

        let translation = &mut transform.translation;
        translation.x += direction.x * it.speed * TIME_STEP;
        translation.y += direction.y * it.speed * TIME_STEP;

        let xmax = params.bounds.x / 2.0 - params.cannon.x / 2.0 - params.wall / 2.0; 
        let ymax = params.bounds.y / 2.0 - params.cannon.y / 2.0 - params.wall / 2.0; 
        translation.x = translation.x.min(xmax).max(-xmax);
        translation.y = translation.y.min(ymax).max(-ymax);
    }
}