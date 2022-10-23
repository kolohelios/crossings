use bevy::{ecs::event::Events, prelude::*, time::Time};
use std::time::Duration;

#[derive(Default)]
struct Clock(Timer);

// We are decoupling the RTC from game time, which is based on ticks; this will give us more flexibility later
#[derive(Default)]
pub struct TicksSinceStart {
    value: u64,
}

fn manage_ticks(mut timer: ResMut<Clock>, mut ticks: ResMut<TicksSinceStart>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("Ticking");
        ticks.value += 1;
    }
}
