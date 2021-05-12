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

pub fn spawn_rope(
    commands: &mut Commands,
    material: Handle<ColorMaterial>,
    ball_point: &Point2<f32>,
    anchor_point: &Point2<f32>,
    ball_entity: Entity,
    anchor_entity: Entity,
) {
    let rope_width = 1.;
    let rope_length = distance(ball_point, anchor_point);
    let middle_point = center(ball_point, anchor_point);
    let angle =
        ((anchor_point.y - ball_point.y).abs() / (anchor_point.x - ball_point.x).abs()).atan();

    let rope_body = RigidBodyBuilder::new_dynamic()
        .rotation(angle)
        .translation(middle_point.x, middle_point.y);

    let rope_collider = ColliderBuilder::cuboid(rope_width / 2., rope_length / 2.);
    let rope_size = Vec2::new(rope_width, rope_length);

    let mut rope_transformation =
        Transform::from_translation(Vec3::new(middle_point.x, middle_point.y, 0.));
    rope_transformation.rotate(Quat::from_rotation_y(angle));

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

    let ball_rope_joint_params =
        BallJoint::new(Point2::new(0., rope_length / 2.), Point2::origin());
    let ball_rope_joint_builder =
        JointBuilderComponent::new(ball_rope_joint_params, rope, ball_entity);
    commands.spawn_bundle((ball_rope_joint_builder,));

    let anchor_rope_joint_params =
        BallJoint::new(Point2::new(0., -(rope_length / 2.)), Point2::origin());
    let anchor_rope_joint_builder =
        JointBuilderComponent::new(anchor_rope_joint_params, rope, anchor_entity);

    commands.spawn_bundle((anchor_rope_joint_builder,));
}

pub fn toggle_rope(
    mut commands: Commands,
    mouse_button: Res<Input<MouseButton>>,
    materials: Res<Materials>,
    rope_query: Query<(Entity, &Rope)>,
    anchor_query: Query<(Entity, &Transform), With<AnchorPoint>>,
    ball_query: Query<(Entity, &Transform), With<Ball>>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    if rope_query.iter().is_empty() {
        // get ball
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
                spawn_rope(
                    &mut commands,
                    materials.rope_material.clone(),
                    &Point2::new(ball.1.translation.x, ball.1.translation.y),
                    &Point2::new(anchor.1.translation.x, anchor.1.translation.y),
                    ball.0,
                    anchor.0,
                )
            }
        }
    } else {
        for (entity, _) in rope_query.iter() {
            commands.entity(entity).despawn();
        }
    }
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
