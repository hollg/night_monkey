use bevy::prelude::*;
use bevy_rapier2d::na::distance;
use bevy_rapier2d::na::Point2;
use bevy_rapier2d::physics::{JointBuilderComponent, RapierPhysicsPlugin};
use bevy_rapier2d::rapier::dynamics::{BallJoint, RigidBodyBuilder};
use bevy_rapier2d::rapier::geometry::ColliderBuilder;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Night Monkey".to_string(),
            width: 700.,
            height: 700.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .add_system(remove_rope.system())
        .run();
}

fn setup_graphics(mut commands: Commands) {
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

struct Rope;

fn setup_physics(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let floor_width = 3.;
    let floor_height = 3.;
    let floor_x = 0.;
    let floor_y = 0.;

    // Static rigid-body with a cuboid shape.
    let floor_body = RigidBodyBuilder::new_static().translation(floor_x, floor_y);

    // let floor_collider = ColliderBuilder::cuboid(floor_width / 2., floor_height / 2.);

    let floor_size = Vec2::new(floor_width, floor_height);
    let floor_material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());

    let floor = commands
        .spawn()
        .insert_bundle((floor_body,))
        .insert_bundle(SpriteBundle {
            material: floor_material,
            sprite: Sprite::new(floor_size),
            ..Default::default()
        })
        .id();

    let ball_diameter = 10.;
    let ball_x = -90.;
    let ball_y = 20.0;

    // Dynamic rigid-body with ball shape.
    let ball_body = RigidBodyBuilder::new_dynamic().translation(ball_x, ball_y);
    let ball_colider = ColliderBuilder::ball(ball_diameter / 2.);

    let ball_size = Vec2::new(ball_diameter, ball_diameter);
    let ball_material = materials.add(Color::rgb(0., 0., 0.).into());

    let ball = commands
        .spawn()
        .insert_bundle((ball_body, ball_colider))
        .insert_bundle(SpriteBundle {
            material: ball_material.clone(),
            sprite: Sprite::new(ball_size),
            ..Default::default()
        })
        .id();

    let rope_width = 1.;
    let rope_length = distance(&Point2::new(floor_x, floor_y), &Point2::new(ball_x, ball_y));
    let rope_body = RigidBodyBuilder::new_dynamic();
    let rope_collider = ColliderBuilder::cuboid(rope_width / 2., rope_length / 2.);
    let rope_size = Vec2::new(rope_width, rope_length);

    let rope = commands
        .spawn()
        .insert(Rope)
        .insert_bundle((rope_body, rope_collider))
        .insert_bundle(SpriteBundle {
            material: ball_material.clone(),
            sprite: Sprite::new(rope_size),
            ..Default::default()
        })
        .id();

    let ball_rope_joint_params =
        BallJoint::new(Point2::new(0., rope_length / 2.), Point2::origin());
    let ball_rope_joint_builder_component =
        JointBuilderComponent::new(ball_rope_joint_params, rope, ball);
    commands.spawn_bundle((ball_rope_joint_builder_component,));

    let floor_rope_joint_params =
        BallJoint::new(Point2::new(0., -(rope_length / 2.)), Point2::origin());
    let ball_rope_joint_builder_component =
        JointBuilderComponent::new(floor_rope_joint_params, rope, floor);

    commands.spawn_bundle((ball_rope_joint_builder_component,));
}

fn remove_rope(
    mut commands: Commands,
    mouse_button: Res<Input<MouseButton>>,
    rope_query: Query<(Entity, &Rope)>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    for (entity, _) in rope_query.iter() {
        commands.entity(entity).despawn();
    }
}
