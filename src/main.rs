use bevy::{prelude::*, winit::WinitSettings};

mod ferry;

fn main() {
    App::new()
        // .insert_resource(WinitSettings::desktop_app())
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

    let sprite_size = 50.0;
    let canvas_size = 30;

    for x in -1 * canvas_size..canvas_size {
        for y in -1 * canvas_size..canvas_size {
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.01 * x as f32, 0.01 * y as f32, 0.75),
                    custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                    ..default()
                },
                transform: Transform::from_xyz(sprite_size * x as f32, sprite_size * y as f32, 0.),
                ..default()
            });
        }
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let translation_distance = 25.0;
    if keyboard_input.pressed(KeyCode::A) {
        for mut transform in camera_query.iter_mut() {
            transform.translation.x -= translation_distance;
            println!("{}", transform.translation.x);
        }
    }
    if keyboard_input.pressed(KeyCode::S) {
        for mut transform in camera_query.iter_mut() {
            transform.translation.y += translation_distance;
        }
    }
    if keyboard_input.pressed(KeyCode::W) {
        for mut transform in camera_query.iter_mut() {
            transform.translation.y -= translation_distance;
        }
    }
    if keyboard_input.pressed(KeyCode::D) {
        for mut transform in camera_query.iter_mut() {
            transform.translation.x += translation_distance;
        }
    }
}
