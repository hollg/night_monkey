use bevy::prelude::*;
use bevy_rapier2d::na::Point2;
use bevy_rapier2d::physics::RapierPhysicsPlugin;
use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier2d::rapier::geometry::ColliderBuilder;

mod rope;
use rope::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Night Monkey".to_string(),
            width: 700.,
            height: 700.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<Materials>()
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
            intensity: 100_000_000.0,
            range: 6000.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(camera);
}

struct Materials {
    ball_material: Handle<ColorMaterial>,
    anchor_point_material: Handle<ColorMaterial>,
    rope_material: Handle<ColorMaterial>,
}

impl FromWorld for Materials {
    fn from_world(world: &mut World) -> Self {
        let mut assets = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        Materials {
            ball_material: assets.add(Color::rgb(0., 0., 0.).into()),
            anchor_point_material: assets.add(Color::rgb(0.5, 0.5, 1.0).into()),
            rope_material: assets.add(Color::rgb(0., 0., 0.).into()),
        }
    }
}

fn setup_physics(mut commands: Commands, materials: Res<Materials>) {
    let floor_x = 0.;
    let floor_y = 0.;
    let floor = spawn_anchor_point(
        &mut commands,
        materials.anchor_point_material.clone(),
        floor_x,
        floor_y,
    );

    let ball_x = -90.;
    let ball_y = 20.;
    let ball = spawn_ball(
        &mut commands,
        materials.ball_material.clone(),
        ball_x,
        ball_y,
    );

    spawn_rope(
        &mut commands,
        materials.rope_material.clone(),
        &Point2::new(floor_x, floor_y),
        &Point2::new(ball_x, ball_y),
        floor,
        ball,
    )
}

fn spawn_anchor_point(
    commands: &mut Commands,
    material: Handle<ColorMaterial>,
    x: f32,
    y: f32,
) -> Entity {
    let width = 3.;
    let height = 3.;

    // Static rigid-body with a cuboid shape.
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
        .id()
}

fn spawn_ball(commands: &mut Commands, material: Handle<ColorMaterial>, x: f32, y: f32) -> Entity {
    let ball_diameter = 10.;
    // let ball_x = -90.;
    // let ball_y = 20.0;

    // Dynamic rigid-body with ball shape.
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
        .id()
}
