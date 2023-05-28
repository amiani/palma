use bevy::prelude::*;
use bevy_fast_tilemap::{FastTileMapPlugin, MapBundle, MeshManagedByMap, Map};
use bevy_rapier2d::prelude::*;
use tiled::Loader;
use vehicle::{Steerable, Wheel, Car};
use bevy::math::{uvec2, vec2};

mod vehicle;

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FastTileMapPlugin)

        .add_startup_system(setup)

        .add_system(steer)
        .add_system(follow_car);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn(Camera2dBundle::default());
    rapier_config.gravity = Vec2::new(0.0, 0.0);
    Car::spawn(&mut commands, asset_server.load("images/big-wheel.png"));

    let map = Map::builder(
        uvec2(500, 350),
        asset_server.load("images/32x32_DEMO.png"),
        vec2(32., 32.),
    )
    .with_perspective_overhang()
    .build_and_initialize(&mut images, |map_indexer| {
        let mut loader = Loader::new();
        let map = loader.load_tmx_map("assets/maps/second.tmx").unwrap();
        let layer = map.get_layer(0).unwrap().as_tile_layer().unwrap();
        let map_size = map_indexer.size();
        for y in 0..map_size.y {
            for x in 0..map_size.x {
                if let Some(tile) = layer.get_tile(x as i32, y as i32) {
                    let index: u16 = tile.id().try_into().unwrap();
                    map_indexer.set(x, y, index);
                }
            }
        }
    });

    commands.spawn(MapBundle::new(map))
        .insert(MeshManagedByMap);
}

pub fn get_facing(rotation: &Quat) -> Vec2 {
	rotation.mul_vec3(Vec3::X).truncate()
}

const DRIVE_FORCE: f32 = 16000.0;
const MAX_SPEED: f32 = 1000.0;
const STEER_TORQUE: f32 = 1.0;

fn steer(
    mut wheels: Query<(&mut ExternalForce, &Velocity, &AdditionalMassProperties, &Transform, Option<&Steerable>)>,
    keys: Res<Input<KeyCode>>
) {
    wheels.for_each_mut(|(mut ext_force, velocity, mass_properties, transform, steerable)| {
        let lateral_friction = Wheel::get_lateral_friction(transform, velocity, mass_properties);
        let facing = get_facing(&transform.rotation);
        let forward_speed = velocity.linvel.dot(facing);

        let drive_force = if keys.pressed(KeyCode::W) && forward_speed < MAX_SPEED {
            facing * DRIVE_FORCE
        }
        else if keys.pressed(KeyCode::S) && forward_speed < MAX_SPEED {
            facing * -DRIVE_FORCE
        }
        else { Vec2::ZERO };

        ext_force.force = drive_force + lateral_friction;

        ext_force.torque = 0.0;
        if steerable.is_none() {
            return;
        }
        if keys.pressed(KeyCode::A) {
            ext_force.torque = STEER_TORQUE;
        }
        else if keys.pressed(KeyCode::D) {
            ext_force.torque = -STEER_TORQUE;
        }
        else {
            //ext_force.torque = transform.rotation.y
            transform.rotation.mul_vec3(Vec3::Z).truncate();
        }
    });
}

fn follow_car(
    mut set: ParamSet<(
        Query<&mut Transform, With<Camera>>,
        Query<&Transform, With<Collider>>
    )>
)
{
    let car_transform = set.p1().single().clone();
    let mut cameras = set.p0();
    let mut camera_transform = cameras.single_mut();
    camera_transform.translation.x = car_transform.translation.x;
    camera_transform.translation.y = car_transform.translation.y;
}