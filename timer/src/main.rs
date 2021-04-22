use bevy::{log::info, prelude::*};
use rand::Rng;
use std::time::Duration;
struct MyTimer(Timer);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(MyTimer(Timer::from_seconds(4.0, false)))
        .add_system(timer_system.system())
        .run();

}

fn timer_system(time: Res<Time>, mut mytimer: ResMut<MyTimer>) {
    mytimer.0.tick(time.delta());
    if mytimer.0.finished() {
        mytimer.0.reset();
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(1..7);
        info!("next timer = {}", roll);
        mytimer.0.set_duration(Duration::from_secs(roll));
    }
}
