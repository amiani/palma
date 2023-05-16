use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0))
        .add_plugin(RapierDebugRenderPlugin::default())

        .add_startup_system(setup)

        .add_system(steer);

    app.run();
}

#[derive(Bundle)]
struct CarBundle {
    pub spatial: SpatialBundle,
    pub body: RigidBody,
    pub collider: Collider,
	pub mass: ColliderMassProperties,
	pub ext_force: ExternalForce,
    pub sprite: Sprite,
    pub texture: Handle<Image>,
}

impl CarBundle {
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            spatial: SpatialBundle::default(),
            body: RigidBody::Dynamic,
            collider: Collider::cuboid(10.0, 5.0),
            mass: ColliderMassProperties::Density(10.0),
            ext_force: ExternalForce::default(),
            sprite: Sprite::default(),
            texture
        }
    }
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());  
    commands.spawn(CarBundle::new(asset_server.load("images/big-wheel.png")));
    rapier_config.gravity = Vec2::new(0.0, 0.0);
}

fn steer(
    mut cars: Query<(&mut ExternalForce, &Transform)>,
    keys: Res<Input<KeyCode>>
) {
    let (mut ext_force, transform) = cars.single_mut();

    if keys.pressed(KeyCode::W) {
        ext_force.force = Vec2::new(100.0, 0.0);
    }
    /*
    if keys.pressed(KeyCode::Down) {
        transform.translation.y -= CAMERA_SPEED;
    }
    if keys.pressed(KeyCode::Right) {
        transform.translation.x += CAMERA_SPEED;
    }
    if keys.pressed(KeyCode::Left) {
        transform.translation.x -= CAMERA_SPEED;
    }
    */
}
