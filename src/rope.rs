use std::cmp::Ordering;

use bevy::prelude::*;
use bevy_rapier2d::na::distance;
use bevy_rapier2d::na::{center, Point2};
use bevy_rapier2d::physics::JointBuilderComponent;
use bevy_rapier2d::rapier::dynamics::{BallJoint, RigidBodyBuilder};
use bevy_rapier2d::rapier::geometry::ColliderBuilder;

use crate::{anchor_point::AnchorPoint, ball::Ball, Materials};

pub struct RopePlugin;

impl Plugin for RopePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(toggle_rope.system());
    }
}
pub struct Rope;
const ROPE_WIDTH: f32 = 1.;

fn toggle_rope(
    mut commands: Commands,
    mouse_button: Res<Input<MouseButton>>,
    materials: Res<Materials>,
    mut rope_query: Query<(Entity, &mut Rope)>,
    anchor_query: Query<(Entity, &Transform), With<AnchorPoint>>,
    ball_query: Query<(Entity, &Transform), With<Ball>>,
) {
    if !mouse_button.is_changed() {
        return;
    }

    if mouse_button.just_released(MouseButton::Left) {
        for rope in rope_query.iter_mut() {
            commands.entity(rope.0).despawn();
        }
    }

    if mouse_button.just_pressed(MouseButton::Left) && rope_query.iter_mut().is_empty() {
        if let Ok((ball_entity, ball_transform)) = ball_query.single() {
            // get closest anchor
            let ball_point =
                Point2::new(ball_transform.translation.x, ball_transform.translation.y);
            let closest_anchor = anchor_query.iter().min_by(|(_, point_a), (_, point_b)| {
                distance(
                    &ball_point,
                    &Point2::new(point_a.translation.x, point_a.translation.y),
                )
                .partial_cmp(&distance(
                    &ball_point,
                    &Point2::new(point_b.translation.x, point_b.translation.y),
                ))
                .unwrap_or(Ordering::Equal)
            });

            if let Some((anchor_entity, anchor_transform)) = closest_anchor {
                // spawn chain between ball and anchor
                let ball_point =
                    Point2::new(ball_transform.translation.x, ball_transform.translation.y);
                let anchor_point = Point2::new(
                    anchor_transform.translation.x,
                    anchor_transform.translation.y,
                );

                spawn_rope(
                    &mut commands,
                    materials.rope_material.clone(),
                    &ball_point,
                    &anchor_point,
                    ball_entity,
                    anchor_entity,
                );
            }
        }
    }
}

/// Spawns a chain of nodes between two entities
fn spawn_rope(
    commands: &mut Commands,
    material: Handle<ColorMaterial>,
    origin_point: &Point2<f32>,
    target_point: &Point2<f32>,
    origin_entity: Entity,
    target_entity: Entity,
) {
    // length and angle of ropes
    // TODO: make one rope longer/shorter if distance isn't divisible by rope_length
    let node_length = 10.;
    let num_nodes = distance(origin_point, target_point) / node_length;
    let chain_angle =
        ((target_point.y - origin_point.y) / (target_point.x - origin_point.x)).atan();

    let mut nodes: Vec<Entity> = vec![];
    let mut current_node_start_point = *origin_point;

    // create all equal-length nodes
    for _ in 0..num_nodes as u8 {
        let current_node_end_point = Point2::new(
            current_node_start_point.x + (node_length * chain_angle.cos()),
            current_node_start_point.y + (node_length * chain_angle.cos()),
        );

        nodes.push(spawn_node(
            commands,
            material.clone(),
            node_length,
            chain_angle,
            &center(&current_node_start_point, &current_node_end_point),
        ));

        current_node_start_point = current_node_end_point;
    }

    let rope_local_start_point = Point2::new(-(node_length / 2.), 0.5);
    let rope_local_end_point = Point2::new(node_length / 2., 0.5);

    // add joint from origin to node_a
    let origin_chain_joint_params = BallJoint::new(
        Point2::origin(),
        if origin_point.x <= target_point.x {
            rope_local_start_point
        } else {
            rope_local_end_point
        },
    );
    let origin_chain_joint_builder =
        JointBuilderComponent::new(origin_chain_joint_params, origin_entity, nodes[0]);
    commands.spawn_bundle((origin_chain_joint_builder,));

    join_nodes(
        commands,
        &nodes,
        rope_local_start_point,
        rope_local_end_point,
        if origin_point.x <= target_point.x {
            DifferenceType::Positive
        } else {
            DifferenceType::Negative
        },
    );

    // add joint from final node to target
    let chain_target_joint_params = BallJoint::new(
        if origin_point.x <= target_point.x {
            rope_local_end_point
        } else {
            rope_local_start_point
        },
        Point2::origin(),
    );
    let chain_target_joint_builder = JointBuilderComponent::new(
        chain_target_joint_params,
        nodes[nodes.len() - 1],
        target_entity,
    );
    commands.spawn_bundle((chain_target_joint_builder,));
}
enum DifferenceType {
    Positive,
    Negative,
}

/// spawns an individual rope node
fn spawn_node(
    commands: &mut Commands,
    material: Handle<ColorMaterial>,
    node_length: f32,
    angle: f32,
    middle_point: &Point2<f32>,
) -> Entity {
    let rope_body = RigidBodyBuilder::new_dynamic()
        .rotation(angle)
        .translation(middle_point.x, middle_point.y);

    let rope_collider = ColliderBuilder::cuboid(node_length / 2., ROPE_WIDTH / 2.);
    let rope_size = Vec2::new(node_length, ROPE_WIDTH);

    let mut rope_transformation =
        Transform::from_translation(Vec3::new(middle_point.x, middle_point.y, 0.));
    rope_transformation.rotate(Quat::from_rotation_z(angle));

    let rope = commands
        .spawn()
        .insert(Rope)
        .insert_bundle((rope_body, rope_collider))
        .insert_bundle(SpriteBundle {
            transform: rope_transformation,
            material,
            sprite: Sprite::new(rope_size),
            ..Default::default()
        })
        .id();

    rope
}

///  spawns joints between each pair of neighbours in a vector of nodes
fn join_nodes(
    commands: &mut Commands,
    nodes: &Vec<Entity>,
    rope_local_start_point: Point2<f32>,
    rope_local_end_point: Point2<f32>,
    difference_type: DifferenceType,
) {
    let positive_diff_rope_joint_params =
        BallJoint::new(rope_local_end_point, rope_local_start_point);
    let negative_diff_rope_joint_params =
        BallJoint::new(rope_local_start_point, rope_local_end_point);

    let mut iter = nodes.windows(2);
    while let Some([node_a, node_b]) = iter.next() {
        let joint_params = match difference_type {
            DifferenceType::Positive => positive_diff_rope_joint_params,
            DifferenceType::Negative => negative_diff_rope_joint_params,
        };

        let joint_builder = JointBuilderComponent::new(joint_params, *node_a, *node_b);
        commands.spawn_bundle((joint_builder,));
    }
}
