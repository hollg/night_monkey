use bevy::prelude::*;

use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier2d::rapier::geometry::ColliderBuilder;

pub struct Ball;
pub fn spawn_ball(
    commands: &mut Commands,
    material: Handle<ColorMaterial>,
    x: f32,
    y: f32,
) -> Entity {
    let ball_diameter = 10.;

    let ball_body = RigidBodyBuilder::new_dynamic().translation(x, y);
    let ball_colider = ColliderBuilder::ball(ball_diameter / 2.);

    let ball_size = Vec2::new(ball_diameter, ball_diameter);
    let ball_material = material;

    commands
        .spawn()
        .insert_bundle((ball_body, ball_colider))
        .insert_bundle(SpriteBundle {
            material: ball_material,
            sprite: Sprite::new(ball_size),
            ..Default::default()
        })
        .insert(Ball)
        .id()
}
