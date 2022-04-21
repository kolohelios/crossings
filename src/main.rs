use bevy::{prelude::*, winit::WinitSettings};

mod ferry;

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
        // .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugin(CrossingsPlugin)
        .run();
}

pub struct CrossingsPlugin;

impl Plugin for CrossingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(keyboard_input_system)
            .add_system(button_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
