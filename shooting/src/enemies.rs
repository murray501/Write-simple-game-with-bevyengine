use crate::{Collider, Params, TIME_STEP, Ball, Scoreboard, Particles};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::Rng;

pub struct EnemyTimer;

pub struct Enemies;

pub struct Enemy {
    speed: Vec2,
    angle: f32,
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
            let speedy = rng.gen_range(-100..100);
            let speedx = rng.gen_range(100..300);
            let size = params.spacejunk.to_owned() * rng.gen_range(0.3..0.5);
            let angle = rng.gen_range(-100..100);
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(params.spacejunk_img.clone().into()),
                transform: Transform::from_xyz(x, y, 1.0),
                sprite: Sprite::new(size),
                ..Default::default()
            })
            .insert(Enemy {speed: Vec2::new(speedx as f32, speedy as f32), angle: angle as f32})
            .insert(Collider::Enemy);

            let interval = rng.gen_range(1..4); 
            timer.set_duration(std::time::Duration::from_secs(interval));
            timer.reset();
        }            
    }
    
    pub fn collision(mut commands: Commands, mut enemies: Query<(Entity, &Sprite, &Transform), With<Enemy>>,
        mut balls: Query<(Entity, &Sprite, &Transform), With<Ball>>, mut scoreboard: ResMut<Scoreboard>,
        particles: Res<Particles>){
        
        for (self_entity, self_sprite, self_transform) in enemies.iter() {
            for (ball_entity, ball_sprite, ball_transform) in balls.iter() {
                let collision = collide(
                    self_transform.translation,
                    self_sprite.size,
                    ball_transform.translation,
                    ball_sprite.size
                );
                if collision.is_some() {
                    let pos = Vec2::new(ball_transform.translation.x, ball_transform.translation.y);
                    commands.entity(self_entity).despawn_recursive();
                    commands.entity(ball_entity).despawn_recursive();
                    scoreboard.score += 1;
                    Particles::spawn(&mut commands, pos, (*particles).clone());
                }
            }
        }
    }

    pub fn update(mut commands: Commands, mut query: Query<(Entity, &mut Enemy, &Sprite, &mut Transform)>, params: Res<Params>){
        let minx = - params.bounds.x * 0.5 + params.wall * 0.5 ;
        
        for (entity, mut enemy, sprite,mut transform) in query.iter_mut() {
            transform.translation.y += enemy.speed.y * TIME_STEP;
            transform.translation.x -= enemy.speed.x * TIME_STEP;

            let maxy = params.bounds.y * 0.5 + sprite.size.y;
            let miny = maxy * -1.0;

            if transform.translation.y < miny {
                transform.translation.y = maxy;
            } else if transform.translation.y > maxy {
                transform.translation.y = miny;
            }

            transform.rotate(Quat::from_rotation_z((enemy.angle * TIME_STEP).to_radians()));

            if transform.translation.x <= (minx + sprite.size.x * 0.5) {
                commands.entity(entity).despawn_recursive();                
            }
        }
    } 
    
    pub fn cleanup(mut commands: Commands, mut query: Query<Entity, With<Enemy>>){
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}