use crate::{Params, TIME_STEP, Ball, Scoreboard, Particles, Collider};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::Rng;

pub struct EnemyTimer;

pub struct Enemies;

#[derive (PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

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
            let mut rng = rand::thread_rng();
            let direction = 
            if rng.gen_range(0.0..1.0) > 0.5 {
                Direction::Left
            } else {
                Direction::Right
            };
            let x =
            if direction == Direction::Left {
                params.background.x * 0.5 + params.cannon.x * 0.5
            } else {
                -params.background.x * 0.5 - params.cannon.x * 0.5
            };
            let ymax = params.background.y * 0.5 - params.cannon.y * 0.5;
            let ymin = -1.0 * ymax;            
            let y = rng.gen_range(ymin..ymax);
            let speedy = rng.gen_range(-100..100);
            let speedx = 
            if direction == Direction::Left {
                rng.gen_range(-300..-100)
            } else {
                rng.gen_range(100..300)
            };
            let size = params.spacejunk.to_owned() * rng.gen_range(0.3..0.5);
            let angle = rng.gen_range(-100..100);
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(params.spacejunk_img.clone().into()),
                transform: Transform::from_xyz(x, y, 1.0),
                sprite: Sprite::new(size),
                ..Default::default()
            })
            .insert(Enemy {speed: Vec2::new(speedx as f32, speedy as f32), angle: angle as f32})
            .insert(Collider::Spacejunk);
            
            let interval = rng.gen_range(1..4); 
            timer.set_duration(std::time::Duration::from_secs(interval));
            timer.reset();
        }            
    }
    
    pub fn update(mut commands: Commands, mut query: Query<(Entity, &mut Enemy, &Sprite, &mut Transform)>, params: Res<Params>){
    
        
        for (entity, mut enemy, sprite,mut transform) in query.iter_mut() {
            transform.translation.y += enemy.speed.y * TIME_STEP;
            transform.translation.x += enemy.speed.x * TIME_STEP;

            let maxy = params.background.y * 0.5 + sprite.size.y * 0.5;
            let miny = maxy * -1.0;

            if transform.translation.y < miny {
                transform.translation.y = maxy;
            } else if transform.translation.y > maxy {
                transform.translation.y = miny;
            }

            transform.rotate(Quat::from_rotation_z((enemy.angle * TIME_STEP).to_radians()));

            let maxx = params.background.x * 0.5 + sprite.size.x * 0.5;
            let minx = -maxx;
        }
    } 
}