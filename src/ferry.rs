use std::time::Duration;
use bevy::{ecs::event::Events, prelude::*, time::Time};

#[derive(Component, Debug)]
pub struct Ferry {
    pub name: String,
}

#[derive(Default)]
struct Clock(Timer);

// We are decoupling the RTC from game time, which is based on ticks; this will give us more flexibility later
#[derive(Default)]
struct TicksSinceStart {
    value: u64,
}

#[derive(Component, Debug)]
struct FerryState {
    status: String,
}

#[derive(Component)]
struct Loaded;

fn spawn_ferry(mut commands: Commands) {
    commands.spawn().insert(Ferry { name: "Wenatchee".to_string() } ).insert(FerryState { status: "docked".to_string() });
}

fn manage_ticks(mut timer: ResMut<Clock>, mut ticks: ResMut<TicksSinceStart>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("Ticking");
        ticks.value += 1;
    }
}

fn prepare_to_load(ticks: Res<TicksSinceStart>, mut query: Query<&mut FerryState>) {
    if ticks.value > 0 {
        for mut ferry_state in query.iter_mut() {
            if ferry_state.status == "docked".to_string() {
                ferry_state.status = "loading".to_string();
            }
        }
    }
}

#[test]
fn spawn_ferry_using_input_resource() {
    // Setup app
    let mut app = App::new();
    let mut time = Time::default();
    time.update();
    app.insert_resource(time);

    app.insert_resource(TicksSinceStart { value: 0 });
    app.insert_resource(Clock(Timer::from_seconds(1.0, true)));

    // Add our systems
    app.add_startup_system(spawn_ferry);
    app.add_system(manage_ticks.before(prepare_to_load));
    app.add_system(prepare_to_load);

    // Run systems
    app.update();

    // Check resulting changes, one entity has been spawned with `Ferry` component that should be docked
    assert_eq!(app.world.query::<&Ferry>().iter(&app.world).len(), 1);
    assert_eq!(app.world.query::<&FerryState>().iter(&app.world).take(1).next().unwrap().status, "docked".to_string());

    // Run systems
    app.update();

    // Check resulting changes, no new entity has been spawned
    assert_eq!(app.world.query::<&Ferry>().iter(&app.world).len(), 1);

    // Simulate that 1s have passed
    let mut time = app.world.resource_mut::<Time>();
    let last_update = time.last_update().unwrap();
    time.update_with_instant(last_update + Duration::from_millis(1001));

    // Run systems
    app.update();

    // Check resulting changes, only one entity has been spawned with `Ferry` component that should be loading
    assert_eq!(app.world.query::<&Ferry>().iter(&app.world).len(), 1);
    assert_eq!(app.world.query::<&FerryState>().iter(&app.world).take(1).next().unwrap().status, "loading".to_string());
}
