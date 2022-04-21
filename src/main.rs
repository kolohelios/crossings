use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CrossingsPlugin)
        .run();
}

pub struct CrossingsPlugin;

impl Plugin for CrossingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(keyboard_input_system);
    }
}

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    commands.spawn_bundle(camera);
    for x in -50..50 {
        for y in -50..50 {
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.01 * x as f32, 0.01 * y as f32, 0.75),
                    custom_size: Some(Vec2::new(25.0, 25.0)),
                    ..default()
                },
                transform: Transform::from_xyz(25.0 * x as f32, 25.0 * y as f32, 0.),
                ..default()
            });
        }
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if keyboard_input.pressed(KeyCode::A) {
        for mut transform in camera_query.iter_mut() {
            transform.translation.x -= 100.0;
        }
    }
    if keyboard_input.pressed(KeyCode::S) {
        for mut transform in camera_query.iter_mut() {
            transform.translation.y += 100.0;
        }
    }
    if keyboard_input.pressed(KeyCode::W) {
        for mut transform in camera_query.iter_mut() {
            transform.translation.y -= 100.0;
        }
    }
    if keyboard_input.pressed(KeyCode::D) {
        for mut transform in camera_query.iter_mut() {
            transform.translation.x += 100.0;
        }
    }
}
