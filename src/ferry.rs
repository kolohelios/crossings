use bevy::prelude::*;

#[derive(Default)]
pub struct TicksSinceStart {
    value: u64,
}

#[derive(Component, Debug)]
pub struct Ferry {
    pub name: String,
}

#[derive(Component, Debug)]
struct FerryState {
    status: String,
    passengers: u16,
    max_passengers: u16,
    next_departure: u64,
}

fn prepare_to_load(ticks: Res<TicksSinceStart>, mut query: Query<&mut FerryState>) {
    for mut ferry_state in query.iter_mut() {
        if ferry_state.status == "docked".to_string() && ferry_state.next_departure <= ticks.value {
            ferry_state.status = "loading".to_string();
        }
    }
}

fn load_ferry(ticks: Res<TicksSinceStart>, mut query: Query<&mut FerryState>) {
    for mut ferry_state in query.iter_mut() {
        if ferry_state.status == "loading".to_string()
            && ferry_state.passengers < ferry_state.max_passengers
        {
            ferry_state.passengers += 100;
        }
    }
}

fn prepare_to_cross(ticks: Res<TicksSinceStart>, mut query: Query<&mut FerryState>) {
    for mut ferry_state in query.iter_mut() {
        if ferry_state.status == "loading".to_string()
            && ferry_state.passengers == ferry_state.max_passengers
        {
            ferry_state.status = "crossing".to_string();
        }
    }
}

#[test]
fn test_prepare_to_load() {
    // Setup app
    let mut app = App::new();

    app.insert_resource(TicksSinceStart { value: 0 });

    fn spawn_ferry(mut commands: Commands) {
        commands
            .spawn()
            .insert(Ferry {
                name: "Wenatchee".to_string(),
            })
            .insert(FerryState {
                status: "docked".to_string(),
                passengers: 0,
                max_passengers: 100,
                next_departure: 1,
            });
    }

    // Add our systems
    app.add_startup_system(spawn_ferry);
    app.add_system(prepare_to_load);

    // Run systems
    app.update();

    // Check resulting changes, one entity has been spawned with `Ferry` component that should be docked
    assert_eq!(app.world.query::<&Ferry>().iter(&app.world).len(), 1);
    assert_eq!(
        app.world
            .query::<&FerryState>()
            .iter(&app.world)
            .take(1)
            .next()
            .unwrap()
            .status,
        "docked".to_string()
    );

    // Run systems
    app.update();

    // Check resulting changes, no new entity has been spawned
    assert_eq!(app.world.query::<&Ferry>().iter(&app.world).len(), 1);

    app.insert_resource(TicksSinceStart { value: 1 });

    // Run systems
    app.update();

    // Check resulting changes, only one entity has been spawned with `Ferry` component that should be loading
    assert_eq!(app.world.query::<&Ferry>().iter(&app.world).len(), 1);
    assert_eq!(
        app.world
            .query::<&FerryState>()
            .iter(&app.world)
            .take(1)
            .next()
            .unwrap()
            .status,
        "loading".to_string()
    );
}

#[test]
fn test_load_ferry() {
    // Setup app
    let mut app = App::new();

    app.insert_resource(TicksSinceStart { value: 0 });

    fn spawn_ferry(mut commands: Commands) {
        commands
            .spawn()
            .insert(Ferry {
                name: "Wenatchee".to_string(),
            })
            .insert(FerryState {
                status: "loading".to_string(),
                passengers: 0,
                max_passengers: 100,
                next_departure: 1,
            });
    }

    // Add our systems
    app.add_startup_system(spawn_ferry);
    app.add_system(load_ferry);

    // Run systems
    app.update();

    assert_eq!(app.world.query::<&Ferry>().iter(&app.world).len(), 1);
    assert_eq!(
        app.world
            .query::<&FerryState>()
            .iter(&app.world)
            .take(1)
            .next()
            .unwrap()
            .passengers,
        100
    );
}

#[test]
fn test_prepare_to_cross() {
    // Setup app
    let mut app = App::new();

    app.insert_resource(TicksSinceStart { value: 0 });

    fn spawn_ferry(mut commands: Commands) {
        commands
            .spawn()
            .insert(Ferry {
                name: "Wenatchee".to_string(),
            })
            .insert(FerryState {
                status: "loading".to_string(),
                passengers: 100,
                max_passengers: 100,
                next_departure: 1,
            });
    }

    // Add our systems
    app.add_startup_system(spawn_ferry);
    app.add_system(prepare_to_cross);

    // Run systems
    app.update();

    // Check resulting changes, only one entity has been spawned with `Ferry` component that should be loading
    assert_eq!(app.world.query::<&Ferry>().iter(&app.world).len(), 1);
    assert_eq!(
        app.world
            .query::<&FerryState>()
            .iter(&app.world)
            .take(1)
            .next()
            .unwrap()
            .status,
        "crossing".to_string()
    );
}
