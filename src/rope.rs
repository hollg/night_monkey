use bevy::prelude::*;
use bevy_rapier2d::na::distance;
use bevy_rapier2d::na::Point2;
use bevy_rapier2d::physics::JointBuilderComponent;
use bevy_rapier2d::rapier::dynamics::{BallJoint, RigidBodyBuilder};
use bevy_rapier2d::rapier::geometry::ColliderBuilder;

pub struct Rope;

pub fn spawn_rope(
    commands: &mut Commands,
    material: Handle<ColorMaterial>,
    point_a: &Point2<f32>,
    point_b: &Point2<f32>,
    entity_a: Entity,
    entity_b: Entity,
) {
    let rope_width = 1.;
    let rope_length = distance(point_a, point_b);
    let rope_body = RigidBodyBuilder::new_dynamic();
    let rope_collider = ColliderBuilder::cuboid(rope_width / 2., rope_length / 2.);
    let rope_size = Vec2::new(rope_width, rope_length);

    let rope = commands
        .spawn()
        .insert(Rope)
        .insert_bundle((rope_body, rope_collider))
        .insert_bundle(SpriteBundle {
            material,
            sprite: Sprite::new(rope_size),
            ..Default::default()
        })
        .id();

    let entity_a_joint_params = BallJoint::new(Point2::new(0., rope_length / 2.), Point2::origin());
    let entity_a_joint_builder = JointBuilderComponent::new(entity_a_joint_params, rope, entity_a);
    commands.spawn_bundle((entity_a_joint_builder,));

    let entity_b_joint_params =
        BallJoint::new(Point2::new(0., -(rope_length / 2.)), Point2::origin());
    let entity_b_joint_builder = JointBuilderComponent::new(entity_b_joint_params, rope, entity_b);

    commands.spawn_bundle((entity_b_joint_builder,));
}

pub fn remove_rope(
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
