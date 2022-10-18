// use std::fs;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    // winit::WinitSettings,
};
// use chrono::{DateTime, Utc};

mod ferry_state;
mod nav;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(CrossingsPlugin)
        .run();
}

pub struct CrossingsPlugin;

impl Plugin for CrossingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            // .insert_resource(WinitSettings::desktop_app())
            .add_startup_system(add_ferries)
            .add_system(keyboard_input_system)
            .add_system(button_system)
            .add_system(text_update_system)
            .add_system(text_color_system)
            .add_system(ferry_status);
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

#[derive(Component)]
struct Ferry;

#[derive(Component, Debug)]
struct Machine(ferry_state::FerryState);

fn add_ferries(mut commands: Commands) {
    let f = ferry_state::ferry_state::create_ferry_state();
    commands.spawn().insert(Ferry).insert(Machine(f));
}

fn ferry_status(query: Query<&Machine, With<Ferry>>) {
    for machine in query.iter() {
        println!("{:?}", machine);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    commands.spawn_bundle(camera);

    let sprite_size = 50.0;
    let canvas_size = 15;

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

    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Button",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..default()
            });
        });

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "hello\nbevy!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(ColorText);
    // Rich text with multiple sections
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(FpsText);
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<bevy::render::camera::Camera2d>>,
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
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}

fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in query.iter_mut() {
        let seconds = time.seconds_since_startup() as f32;
        // We used the `Text::with_section` helper method, but it is still just a `Text`,
        // so to update it, we are still updating the one and only section
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}
