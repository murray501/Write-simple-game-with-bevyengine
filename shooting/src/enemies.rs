use crate::{Collider, Params, TIME_STEP, Ball, Scoreboard};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::Rng;

pub struct EnemyTimer;

pub struct Enemies;

pub struct Enemy {
    speed: Vec2,
}

impl Enemies {
    pub fn spawner(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, 
        params: Res<Params>, mut query_timer: Query<&mut Timer, With<EnemyTimer>>, time: Res<Time>)
    {
        let mut can_spawn = false;
        let mut timer = query_timer.single_mut().unwrap();
        if timer.tick(time.delta()).finished() {
            can_spawn = true;
        } 

        if can_spawn {
            let x = params.bounds.x * 0.5 - params.wall * 0.5 - 15.0;
            let ymax = params.bounds.y * 0.5 - params.wall * 0.5 - 15.0;
            let ymin = -1.0 * ymax;
            let mut rng = rand::thread_rng();
            let y = rng.gen_range(ymin..ymax);
            let speedy = rng.gen_range(-500..500);
            let speedx = rng.gen_range(100..500);
            let size = rng.gen_range(5..100);
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(0.0, 0.5, 0.5).into()),
                transform: Transform::from_xyz(x,y, 1.0),
                sprite: Sprite::new(Vec2::new(size as f32, size as f32)),
                ..Default::default()
            })
            .insert(Enemy {speed: Vec2::new(speedx as f32, speedy as f32)})
            .insert(Collider::Enemy);

            let interval = rng.gen_range(1..4); 
            timer.set_duration(std::time::Duration::from_secs(interval));
            timer.reset();
        }            
    }
    
    pub fn collision(mut commands: Commands, mut enemies: Query<(Entity, &Sprite, &Transform), With<Enemy>>,
        mut balls: Query<(Entity, &Sprite, &Transform), With<Ball>>, mut scoreboard: ResMut<Scoreboard>){
        
        for (self_entity, self_sprite, self_transform) in enemies.iter() {
            for (ball_entity, ball_sprite, ball_transform) in balls.iter() {
                let collision = collide(
                    self_transform.translation,
                    self_sprite.size,
                    ball_transform.translation,
                    ball_sprite.size
                );
                if collision.is_some() {
                    commands.entity(self_entity).despawn();
                    commands.entity(ball_entity).despawn();
                    scoreboard.score += 1;
                }
            }
        }
    }

    pub fn update(mut commands: Commands, mut query: Query<(Entity, &mut Enemy, &Sprite, &mut Transform)>, params: Res<Params>){
        let minx = - params.bounds.x * 0.5 + params.wall * 0.5 ;
        
        for (entity, mut enemy, sprite,mut transform) in query.iter_mut() {
            transform.translation.y += enemy.speed.y * TIME_STEP;
            transform.translation.x -= enemy.speed.x * TIME_STEP;

            let maxy = params.bounds.y * 0.5 - params.wall * 0.5 - sprite.size.y * 0.5 ;
            let miny = maxy * -1.0;

            if transform.translation.y <= miny {
                enemy.speed.y *= -1.0;
            } else if transform.translation.y >= maxy {
                enemy.speed.y *= -1.0;
            }

            if transform.translation.x <= (minx + sprite.size.x * 0.5) {
                commands.entity(entity).despawn();                
            }
        }
    } 
    
    pub fn cleanup(mut commands: Commands, mut query: Query<Entity, With<Enemy>>){
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}