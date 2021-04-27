use bevy::prelude::*;
pub const TIME_STEP: f32 = 1.0 / 60.0;
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(handle_mouse_clicks.system())
        .add_system(Particles::update.system())
        .run();
}

fn handle_mouse_clicks(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    let win = windows.get_primary().unwrap();
    if mouse_input.just_pressed(MouseButton::Left) {
        let cursor_position = win.cursor_position().unwrap();
        let size = Vec2::new(win.width() as f32, win.height() as f32);
        let pos = cursor_position - size * 0.5;
        Particles::spawn(commands, materials, pos, 8, 100.0);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

struct Particles;

struct Particle {
    speed: Vec2,
}

impl Particles {
    fn spawn(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        pos: Vec2,
        num_divide: usize,
        speed: f32,
    ) {
        let unit = std::f32::consts::PI * 2.0 / num_divide as f32;

        for i in 0..num_divide {
            let angle = unit * i as f32;
            let speed_vec = Vec2::new(angle.cos(), angle.sin()) * speed;
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    transform: Transform::from_xyz(pos.x, pos.y, 1.0),
                    sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                })
                .insert(Particle{ speed: speed_vec });
        }
    }

    fn update(mut commands: Commands, mut query: Query<(Entity, &Particle, &Sprite, &mut Transform)>){
        for (entity, particle, sprite, mut transform) in query.iter_mut() {
            transform.translation.x += particle.speed.x * TIME_STEP;
            transform.translation.y += particle.speed.y * TIME_STEP;
            transform.scale *= 0.99;
            let size = sprite.size.x * transform.scale.x;
            if size < 5.0 {
                commands.entity(entity).despawn();
            }
        }
    } 
}
