use bevy::{prelude::*, time::FixedTimestep};
use bevy_turborand::*;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;
const LOGO_SIZE: Vec2 = Vec2::new(256.0, 159.0);

#[derive(Component)]
struct DvdLogo;

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Direction(Vec2);

#[derive(Debug)]
struct WindowSize {
    width: f32,
    height: f32,
}

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
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(DvdLogoPlugin::apply_speed)
                    .with_system(DvdLogoPlugin::bounce)
                    .with_system(DvdLogoPlugin::change_color),
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
        commands.spawn_bundle(Camera2dBundle::default());
        commands
            .spawn()
            .insert(DvdLogo)
            .insert(Speed(100.0))
            .insert(Direction(Vec2::new(1.0, 1.0)))
            .insert(RngComponent::from(&mut global_rng))
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("dvd.png"),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::ANTIQUE_WHITE,
                    ..Default::default()
                },
                ..default()
            });

        let window = windows.get_primary().unwrap();
        commands.insert_resource(WindowSize {
            height: window.height(),
            width: window.width(),
        })
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
    ) {
        let (transform, mut direction) = logo_query.single_mut();
        let (x, y) = (transform.translation.x, transform.translation.y);


        let right_bound = window_size.width / 2.0 - LOGO_SIZE.x / 2.0;
        let left_bound = -right_bound;

        let top_bound = window_size.height / 2.0 - LOGO_SIZE.y / 2.0;
        let bottom_bound = -top_bound;

        if x >= right_bound || x <= left_bound {
            direction.0.x *= -1.0;
        }
        if y >= top_bound || y <= bottom_bound {
            direction.0.y *= -1.0;
        }
    }
}
