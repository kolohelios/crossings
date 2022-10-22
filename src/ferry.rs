use bevy::{ecs::event::Events, prelude::*};

#[derive(Component, Default)]
pub struct Ferry {
    pub name: String,
}

#[derive(Default)]
struct Clock(Timer);

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Docked;

struct Loading(bool);

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Crossing;


#[derive(Component)]
#[component(storage = "SparseSet")]
struct Unloading;

fn update_ferry_state(mut loading: EventReader<Loading>) {
    for _value in loading.iter() {
        // Do something
    }
}

fn spawn_ferry(mut commands: Commands) {
    commands.spawn().insert(Ferry { name: "Wenatchee".to_string() } ).insert(Docked);
}

#[test]
fn spawn_ferry_using_input_resource() {
    // Setup app
    let mut app = App::new();

    app.add_event::<Loading>();
    app.insert_resource(Clock(Timer::from_seconds(2.0, true)));

    // Add our systems
    app.add_startup_system(spawn_ferry);
    app.add_startup_system(update_ferry_state);

    // Run systems
    app.update();

    // Check resulting changes, one entity has been spawned with `Ferry` component
    assert_eq!(app.world.query::<&Ferry>().iter(&app.world).len(), 1);
    // Should be docked
    assert_eq!(app.world.query::<&Docked>().iter(&app.world).len(), 1);

    // Run systems
    app.update();

    // Check resulting changes, no new entity has been spawned
    assert_eq!(app.world.query::<&Ferry>().iter(&app.world).len(), 1);
}
