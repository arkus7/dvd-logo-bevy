#![allow(clippy::type_complexity)]

use bevy::{prelude::*, time::FixedTimestep};
use bevy_turborand::*;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;
const LOGO_SIZE: Vec2 = Vec2::new(256.0, 159.0);
const INITIAL_SPEED: f32 = 150.0;

#[derive(Component)]
struct DvdLogo;

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Direction(Vec2);

#[derive(Debug, Clone, Copy)]
struct WindowSize {
    width: f32,
    height: f32,
}

#[derive(Debug, Default)]
struct CollisionEvent;

struct CollisionSound(Handle<AudioSource>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DvdLogoPlugin)
        .run();
}

struct DvdLogoPlugin;

impl Plugin for DvdLogoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RngPlugin::default())
            .add_startup_system(DvdLogoPlugin::setup)
            .add_event::<CollisionEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(DvdLogoPlugin::apply_speed)
                    .with_system(DvdLogoPlugin::bounce)
                    .with_system(DvdLogoPlugin::change_color)
                    .with_system(DvdLogoPlugin::play_collision_sound)
                    .with_system(DvdLogoPlugin::change_speed)
            );
    }
}

impl DvdLogoPlugin {
    fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        windows: Res<Windows>,
        mut global_rng: ResMut<GlobalRng>,
    ) {
        let window = windows.get_primary().unwrap();
        let window_size = WindowSize {
            height: window.height(),
            width: window.width(),
        };
        commands.insert_resource(window_size);

        let random_pos = Vec3::new(
            global_rng.f32() * window_size.width / 2.0 - LOGO_SIZE.x,
            global_rng.f32() * window_size.height / 2.0 - LOGO_SIZE.y,
            0.0,
        );

        commands.spawn_bundle(Camera2dBundle::default());
        commands
            .spawn()
            .insert(DvdLogo)
            .insert(Speed(INITIAL_SPEED))
            .insert(Direction(Vec2::new(1.0, 1.0)))
            .insert(RngComponent::from(&mut global_rng))
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("dvd.png"),
                transform: Transform {
                    translation: random_pos,
                    ..default()
                },
                ..default()
            });

        let collision_sound = asset_server.load("sounds/meow.ogg");
        commands.insert_resource(CollisionSound(collision_sound));
    }

    fn apply_speed(mut query: Query<(&mut Transform, &Speed, &Direction)>) {
        for (mut transform, speed, direction) in &mut query {
            transform.translation.x += direction.0.x * speed.0 * TIME_STEP;
            transform.translation.y += direction.0.y * speed.0 * TIME_STEP;
        }
    }

    fn change_color(
        mut query: Query<(&mut Sprite, &mut RngComponent), (With<DvdLogo>, Changed<Direction>)>,
    ) {
        for (mut sprite, mut rng) in query.iter_mut() {
            let color = Color::rgb(rng.f32(), rng.f32(), rng.f32());
            sprite.color = color;
        }
    }

    fn bounce(
        window_size: Res<WindowSize>,
        mut logo_query: Query<(&mut Transform, &mut Direction), With<DvdLogo>>,
        mut collision_events: EventWriter<CollisionEvent>,
    ) {
        let (transform, mut direction) = logo_query.single_mut();
        let (x, y) = (transform.translation.x, transform.translation.y);

        let right_bound = window_size.width / 2.0 - LOGO_SIZE.x / 2.0;
        let left_bound = -right_bound;

        let top_bound = window_size.height / 2.0 - LOGO_SIZE.y / 2.0;
        let bottom_bound = -top_bound;

        let bounce_horizontally = x >= right_bound || x <= left_bound;
        let bounce_vertically = y >= top_bound || y <= bottom_bound;

        match (bounce_horizontally, bounce_vertically) {
            (true, true) => {
                direction.0 *= -1.0;
                collision_events.send_default();
            }
            (true, _) => {
                direction.0.x *= -1.0;
            }
            (_, true) => {
                direction.0.y *= -1.0;
            }
            _ => {}
        }
    }

    fn play_collision_sound(
        collision_events: EventReader<CollisionEvent>,
        audio: Res<Audio>,
        sound: Res<CollisionSound>,
    ) {
        // Play a sound once per frame if a collision for both axis occurred.
        if !collision_events.is_empty() {
            // This prevents events staying active on the next frame.
            collision_events.clear();
            audio.play(sound.0.clone());
        }
    }

    fn change_speed(
        mut query: Query<&mut Speed, With<DvdLogo>>,
        keyboard_input: Res<Input<KeyCode>>,
    ) {
        let mut logo_speed = query.single_mut();

        if keyboard_input.pressed(KeyCode::Right) {
            logo_speed.0 += 10.0;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            logo_speed.0 -= 10.0;
        }
    }
}
