use bevy::prelude::*;
use bevy_rapier2d::physics::{RapierConfiguration, RapierPhysicsPlugin};

use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier2d::rapier::geometry::ColliderBuilder;
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 700.,
            height: 700.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .run();
}

fn setup_graphics(mut commands: Commands, mut configuration: ResMut<RapierConfiguration>) {
    configuration.scale = 10.0;

    let camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(1000.0, 10.0, 2000.0)),
        light: Light {
            intensity: 100_000_000_.0,
            range: 6000.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(camera);
}

fn setup_physics(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let floor_width = 30.;
    let floor_height = 10.;
    let floor_x = 0.;
    let floor_y = 0.;

    // Static rigid-body with a cuboid shape.
    let static_body = RigidBodyBuilder::new_static().translation(floor_x, floor_y);
    let floor_collider = ColliderBuilder::cuboid(floor_width / 2., floor_height / 2.);

    let floor_size = Vec2::new(floor_width, floor_height);
    let floor_material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    let floor_position = Vec3::new(floor_x, floor_y, 1.0);

    commands
        .spawn()
        .insert_bundle((static_body, floor_collider))
        .insert_bundle(SpriteBundle {
            material: floor_material,
            sprite: Sprite::new(floor_size),
            transform: Transform::from_translation(floor_position),
            ..Default::default()
        });

    let ball_diameter = 5.;
    let ball_x = 0.;
    let ball_y = 20.0;

    // Dynamic rigid-body with ball shape.
    let dynamic_body = RigidBodyBuilder::new_dynamic().translation(ball_x, ball_y);
    let ball_colider = ColliderBuilder::ball(ball_diameter / 2.);

    let ball_size = Vec2::new(ball_diameter, ball_diameter);
    let ball_material = materials.add(Color::rgb(0.7, 0.2, 1.0).into());
    let ball_position = Vec3::new(ball_x, ball_y, 1.0);

    commands
        .spawn()
        .insert_bundle((dynamic_body, ball_colider))
        .insert_bundle(SpriteBundle {
            material: ball_material,
            sprite: Sprite::new(ball_size),
            transform: Transform::from_translation(ball_position),
            ..Default::default()
        });
}
