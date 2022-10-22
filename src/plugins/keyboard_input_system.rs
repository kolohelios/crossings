use bevy::prelude::*;

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let translation_distance = 25.0;
    let scale_amount = 0.1;
    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        for mut transform in camera_query.iter_mut() {
            transform.translation.x += translation_distance;
        }
    }
    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        for mut transform in camera_query.iter_mut() {
            transform.translation.y += translation_distance;
        }
    }
    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        for mut transform in camera_query.iter_mut() {
            transform.translation.y -= translation_distance;
        }
    }
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        for mut transform in camera_query.iter_mut() {
            transform.translation.x -= translation_distance;
        }
    }
    if keyboard_input.pressed(KeyCode::Equals) {
        for mut transform in camera_query.iter_mut() {
            transform.scale.x += scale_amount;
            transform.scale.y += scale_amount;
        }
    }
    if keyboard_input.pressed(KeyCode::Minus) {
        for mut transform in camera_query.iter_mut() {
            transform.scale.x -= scale_amount;
            transform.scale.y -= scale_amount;
        }
    }

    if keyboard_input.pressed(KeyCode::Escape) {
        // show a menu here
    }
}
