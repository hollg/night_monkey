#![feature(exact_size_is_empty)]
use bevy::prelude::*;
use bevy_rapier2d::physics::RapierPhysicsPlugin;

mod anchor_point;
use anchor_point::spawn_anchor_point;

mod ball;
use ball::spawn_ball;

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
        .add_plugin(RopePlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_objects.system())
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

pub struct Materials {
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

fn setup_objects(mut commands: Commands, materials: Res<Materials>) {
    let anchors: Vec<(f32, f32)> = vec![(-200., 0.), (0., 0.), (200.0, 0.)];

    for (x, y) in anchors.iter() {
        spawn_anchor_point(
            &mut commands,
            materials.anchor_point_material.clone(),
            *x,
            *y,
        );
    }

    let ball_x = -250.;
    let ball_y = 50.;
    spawn_ball(
        &mut commands,
        materials.ball_material.clone(),
        ball_x,
        ball_y,
    );
}
