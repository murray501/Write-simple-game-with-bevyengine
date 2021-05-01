use crate::{Params, TIME_STEP, Balls, Scoreboard, Particles, Cannon, Collider};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::Rng;

pub struct EnemyShipTimer;
pub struct EnemyShotTimer;

pub struct EnemyShips;

pub struct EnemyShip {
    speed: f32,
    direction: Vec2,
}

impl EnemyShips {
    pub fn spawner(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>,
        params: Res<Params>, mut query_timer: Query<&mut Timer, With<EnemyShipTimer>>, 
        time: Res<Time>, query_cannon: Query<&Transform, With<Cannon>>)
    {
        let mut can_spawn = false;
        let mut timer = query_timer.single_mut().unwrap();
        
        if timer.tick(time.delta()).finished() {
            can_spawn = true;
        } 
        if !can_spawn {
            return;
        }

        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0.0..1.0) as f32;
        let maxx = params.background.x * 0.5;
        let maxy = params.background.y * 0.5;
        let rand_x = rng.gen_range(-maxx..maxx);
        let rand_y = rng.gen_range(-maxy..maxy);
        let pos = 
            if random < 0.25 {
                Vec2::new(-maxx, rand_y)
            } else if random < 0.5 {
                Vec2::new(maxx, rand_y)
            } else if random < 0.75 {
                Vec2::new(rand_x, -maxy)
            } else {
                Vec2::new(rand_x, maxy)
            };
        
        let cannon_transform = query_cannon.single().unwrap();
        let cannon_pos = Vec2::new(cannon_transform.translation.x, cannon_transform.translation.y);
        let direction = (cannon_pos - pos).normalize();
        let radian = direction.y.atan2(direction.x) + std::f32::consts::PI;
        let size = params.enemyship;
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(params.enemyship_img.clone().into()),
                transform: Transform {
                    translation: Vec3::new(pos.x, pos.y, 3.0),
                    rotation: Quat::from_rotation_z(radian),
                    ..Default::default()
                },
                sprite: Sprite::new(size),
                ..Default::default()
            })
            .insert(EnemyShip {speed: 100.0, direction})
            .insert(Collider::Enemyship);
            
            let interval = rng.gen_range(5..10); 
            timer.set_duration(std::time::Duration::from_secs(interval));
            timer.reset();                    
    }
    
    pub fn update(mut commands: Commands, mut query: Query<(Entity, &EnemyShip, &mut Transform)>, 
        params: Res<Params>)
    {  
        for (entity, enemy, mut transform) in query.iter_mut() {
            transform.translation.y += enemy.direction.y * enemy.speed * TIME_STEP;
            transform.translation.x += enemy.direction.x * enemy.speed * TIME_STEP;
        }
    } 
    
    pub fn shoot(mut commands: Commands, query: Query<(&Transform, &EnemyShip), 
        With<EnemyShip>>, params: Res<Params>,time: Res<Time>,
        mut query_timer: Query<&mut Timer, With<EnemyShotTimer>>)
    {
        let mut timer = query_timer.single_mut().unwrap();
        if !timer.tick(time.delta()).just_finished() {
            return;
        } 
        for (transform, enemyship) in query.iter() {
            let pos = Vec2::new(transform.translation.x, transform.translation.y);
            Balls::spawn(&mut commands, pos, enemyship.direction.to_owned(), &params);
        }
    }
}