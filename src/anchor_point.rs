use bevy::prelude::*;
use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;

pub struct AnchorPoint;

pub fn spawn_anchor_point(
    commands: &mut Commands,
    material: Handle<ColorMaterial>,
    x: f32,
    y: f32,
) -> Entity {
    let width = 3.;
    let height = 3.;
    let body = RigidBodyBuilder::new_static().translation(x, y);
    let sprite_size = Vec2::new(width, height);

    commands
        .spawn()
        .insert_bundle((body,))
        .insert_bundle(SpriteBundle {
            material,
            sprite: Sprite::new(sprite_size),
            ..Default::default()
        })
        .insert(AnchorPoint)
        .id()
}
