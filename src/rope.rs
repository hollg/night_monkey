use std::cmp::Ordering;

use bevy::prelude::*;
use bevy_rapier2d::na::{center, Point2};
use bevy_rapier2d::physics::JointBuilderComponent;
use bevy_rapier2d::rapier::dynamics::{BallJoint, RigidBodyBuilder};
use bevy_rapier2d::rapier::geometry::ColliderBuilder;
use bevy_rapier2d::{na::distance, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBody};

use crate::{anchor_point::AnchorPoint, ball::Ball, Materials};

pub struct RopePlugin;

impl Plugin for RopePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(toggle_rope.system());
    }
}
pub struct Rope;
const ROPE_WIDTH: f32 = 1.;

pub fn spawn_chain(
    commands: &mut Commands,
    material: Handle<ColorMaterial>,
    origin_point: &Point2<f32>,
    target_point: &Point2<f32>,
    origin_entity: Entity,
    target_entity: Entity,
) {
    // find center point between origin and target
    let middle_point = center(origin_point, target_point);

    // find length and angle of ropes
    let rope_length = distance(origin_point, target_point) / 2.;
    let rope_angle = ((target_point.y - origin_point.y) / (target_point.x - origin_point.x)).atan();

    // spawn rope between origin and center
    let node_a = spawn_rope(
        commands,
        material.clone(),
        origin_point,
        &middle_point,
        origin_entity,
        rope_length,
        rope_angle,
        &center(&origin_point, &middle_point),
    );

    // spawn rope between center and target
    let node_b = spawn_rope(
        commands,
        material.clone(),
        &middle_point,
        target_point,
        node_a,
        rope_length,
        rope_angle,
        &center(&middle_point, target_point),
    );

    // add joint from origin to node_a
    let ball_node_a_joint_params = BallJoint::new(
        Point2::origin(),
        if origin_point.x <= target_point.x {
            Point2::new(-(rope_length / 2.), 0.5)
        } else {
            Point2::new(rope_length / 2., 0.5)
        },
    );
    let ball_node_a_joint_builder =
        JointBuilderComponent::new(ball_node_a_joint_params, origin_entity, node_a);
    commands.spawn_bundle((ball_node_a_joint_builder,));

    // add joint between ropes
    let node_a_node_b_joint_params = BallJoint::new(
        if origin_point.x <= target_point.x {
            Point2::new(rope_length / 2., 0.5)
        } else {
            Point2::new(-(rope_length / 2.), 0.5)
        },
        if origin_point.x <= target_point.x {
            Point2::new(-(rope_length / 2.), 0.5)
        } else {
            Point2::new(rope_length / 2., 0.5)
        },
    );
    let node_a_node_b_joint_builder =
        JointBuilderComponent::new(node_a_node_b_joint_params, node_a, node_b);
    commands.spawn_bundle((node_a_node_b_joint_builder,));

    // add joint from node_b to target
    let node_b_target_joint_parms = BallJoint::new(
        if origin_point.x <= target_point.x {
            Point2::new(rope_length / 2., 0.5)
        } else {
            Point2::new(-(rope_length / 2.), 0.5)
        },
        Point2::origin(),
    );
    let node_b_target_joint_builder =
        JointBuilderComponent::new(node_b_target_joint_parms, node_b, target_entity);
    commands.spawn_bundle((node_b_target_joint_builder,));
}

pub fn spawn_rope(
    commands: &mut Commands,
    material: Handle<ColorMaterial>,
    origin_point: &Point2<f32>,
    target_point: &Point2<f32>,
    origin_entity: Entity,
    // target_entity: Entity,
    rope_length: f32,
    rope_angle: f32,
    middle_point: &Point2<f32>,
) -> Entity {
    // let rope_length = distance(origin_point, target_point);
    // let middle_point = center(origin_point, target_point);
    // let angle = ((target_point.y - origin_point.y) / (target_point.x - origin_point.x)).atan();

    let rope_body = RigidBodyBuilder::new_dynamic()
        .rotation(rope_angle)
        .translation(middle_point.x, middle_point.y);

    let rope_collider = ColliderBuilder::cuboid(rope_length / 2., ROPE_WIDTH / 2.);
    let rope_size = Vec2::new(rope_length, ROPE_WIDTH);

    let mut rope_transformation =
        Transform::from_translation(Vec3::new(middle_point.x, middle_point.y, 0.));
    rope_transformation.rotate(Quat::from_rotation_z(rope_angle));

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

    return rope;
}

pub fn toggle_rope(
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

    if mouse_button.just_pressed(MouseButton::Left) {
        if rope_query.iter_mut().is_empty() {
            if let Ok(ball) = ball_query.single() {
                // get closest anchor
                let ball_point = Point2::new(ball.1.translation.x, ball.1.translation.y);
                let closest_anchor = anchor_query.iter().min_by(|(_, x), (_, y)| {
                    distance(&ball_point, &Point2::new(x.translation.x, x.translation.y))
                        .partial_cmp(&distance(
                            &ball_point,
                            &Point2::new(y.translation.x, y.translation.y),
                        ))
                        .unwrap_or(Ordering::Equal)
                });

                if let Some(anchor) = closest_anchor {
                    // spawn rope between ball and anchor
                    let origin_point = Point2::new(ball.1.translation.x, ball.1.translation.y);
                    let anchor_point = Point2::new(anchor.1.translation.x, anchor.1.translation.y);
                    let rope_length = distance(&origin_point, &anchor_point);
                    let rope_angle = ((anchor_point.y - origin_point.y)
                        / (anchor_point.x - origin_point.x))
                        .atan();

                    spawn_chain(
                        &mut commands,
                        materials.rope_material.clone(),
                        &origin_point,
                        &anchor_point,
                        ball.0,
                        anchor.0,
                    );

                    // let rope = spawn_rope(
                    //     &mut commands,
                    //     materials.rope_material.clone(),
                    //     &origin_point,
                    //     &anchor_point,
                    //     ball.0,
                    //     // anchor.0,
                    //     rope_length,
                    //     rope_angle,
                    //     &center(&origin_point, &anchor_point),
                    // );

                    // let ball_rope_joint_params =
                    //     BallJoint::new(Point2::origin(), Point2::new(-(rope_length / 2.), 0.5));
                    // let ball_rope_joint_builder = JointBuilderComponent::new(
                    //     ball_rope_joint_params,
                    //     if origin_point.x <= anchor_point.x {
                    //         ball.0
                    //     } else {
                    //         anchor.0
                    //     },
                    //     rope,
                    // );
                    // commands.spawn_bundle((ball_rope_joint_builder,));

                    // let anchor_rope_joint_params =
                    //     BallJoint::new(Point2::new(rope_length / 2., 0.5), Point2::origin());
                    // let anchor_rope_joint_builder = JointBuilderComponent::new(
                    //     anchor_rope_joint_params,
                    //     rope,
                    //     if origin_point.x <= anchor_point.x {
                    //         anchor.0
                    //     } else {
                    //         ball.0
                    //     },
                    // );

                    // commands.spawn_bundle((anchor_rope_joint_builder,));
                }
            }
        }
    }

    // if mouse_button.pressed(MouseButton::Left) &&

    // if rope_query.iter().is_empty() {
    //     // get ball
    //     if let Ok(ball) = ball_query.single() {
    //         // get closest anchor
    //         let ball_point = Point2::new(ball.1.translation.x, ball.1.translation.y);
    //         let closest_anchor = anchor_query.iter().min_by(|(_, x), (_, y)| {
    //             distance(&ball_point, &Point2::new(x.translation.x, x.translation.y))
    //                 .partial_cmp(&distance(
    //                     &ball_point,
    //                     &Point2::new(y.translation.x, y.translation.y),
    //                 ))
    //                 .unwrap_or(Ordering::Equal)
    //         });

    //         if let Some(anchor) = closest_anchor {
    //             // spawn rope between ball and anchor
    //             spawn_rope(
    //                 &mut commands,
    //                 materials.rope_material.clone(),
    //                 &Point2::new(ball.1.translation.x, ball.1.translation.y),
    //                 &Point2::new(anchor.1.translation.x, anchor.1.translation.y),
    //                 ball.0,
    //                 anchor.0,
    //             )
    //         }
    //     }
    // } else {
    //     for (entity, _) in rope_query.iter() {
    //         commands.entity(entity).despawn();
    //     }
    // }
}

fn zip_to_point(
    anchor_query: Query<(Entity, &Transform), With<AnchorPoint>>,
    mut ball_query: Query<(Entity, &mut Transform), With<Ball>>,
    keys: Res<Input<KeyCode>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    // get ball
    let mut ball = ball_query.single_mut().unwrap();

    // get closest anchor
    let ball_point = Point2::new(ball.1.translation.x, ball.1.translation.y);
    let closest_anchor = anchor_query
        .iter()
        .min_by(|(_, x), (_, y)| {
            distance(&ball_point, &Point2::new(x.translation.x, x.translation.y))
                .partial_cmp(&distance(
                    &ball_point,
                    &Point2::new(y.translation.x, y.translation.y),
                ))
                .unwrap_or(Ordering::Equal)
        })
        .unwrap();

    let closest_anchor_point = Point2::new(
        closest_anchor.1.translation.x,
        closest_anchor.1.translation.y,
    );
}
