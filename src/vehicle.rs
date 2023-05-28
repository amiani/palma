use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Steerable;

#[derive(Bundle)]
pub struct Car {
    pub spatial: SpatialBundle,
    pub body: RigidBody,
    pub collider: Collider,
	pub mass: ColliderMassProperties,
    pub sprite: Sprite,
    pub texture: Handle<Image>,
}


impl Car {
    fn new(texture: Handle<Image>) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 10.0)),
            body: RigidBody::Dynamic,
            collider: Collider::cuboid(64.0, 32.0),
            mass: ColliderMassProperties::Density(10.0),
            sprite: Sprite::default(),
            texture
        }
    }

	pub fn spawn(commands: &mut Commands, texture: Handle<Image>) {
		let car = commands.spawn(Car::new(texture)).id();

		let front_left_anchor = Vec2::new(60.0, 25.0);
		commands
			.spawn(Wheel::new(car, front_left_anchor, true))
			.insert(Steerable);

		let front_right_anchor = Vec2::new(60.0, -25.0);
		commands
			.spawn(Wheel::new(car, front_right_anchor, true))
			.insert(Steerable);
		
		let back_left_anchor = Vec2::new(-60.0, 25.0);
		commands.spawn(Wheel::new(car, back_left_anchor, false));

		let back_right_anchor = Vec2::new(-60.0, -25.0);
		commands.spawn(Wheel::new(car, back_right_anchor, false));
	}
}

#[derive(Bundle)]
pub struct Wheel {
	pub spatial: SpatialBundle,
	pub body: RigidBody,
	pub ext_force: ExternalForce,
	pub velocity: Velocity,
	pub mass: AdditionalMassProperties,
	pub joint: ImpulseJoint,
}

impl Wheel {
	pub fn new(parent: Entity, parent_anchor: Vec2, steerable: bool) -> Self {
		let joint = if steerable {
			ImpulseJoint::new(
				parent,
				RevoluteJointBuilder::new()
						.local_anchor1(parent_anchor)
						.local_anchor2(Vec2::ZERO)
						.limits([-PI / 3.0, PI / 3.0])
			)
		} else {
			ImpulseJoint::new(
				parent,
				FixedJointBuilder::new()
					.local_anchor1(parent_anchor)
					.local_anchor2(Vec2::ZERO)
			)
		};
		let mass = MassProperties {
			local_center_of_mass: Vec2::default(),
			mass: 10.0,
			principal_inertia: 1.5,
		};
		Self {
			spatial: SpatialBundle::default(),
			body: RigidBody::Dynamic,
			ext_force: ExternalForce::default(),
			velocity: Velocity::default(),
			mass: AdditionalMassProperties::MassProperties(mass),
			joint
		}
	}

	/**
     * Kills the lateral movement of the wheel. Returns a force that will stop the lateral motion of the wheel in one timestep.
     */
	pub fn get_lateral_friction(transform: &Transform, velocity: &Velocity, mass_properties: &AdditionalMassProperties) -> Vec2 {
		let lateral_normal = transform.down().truncate();
		let lateral_velocity = lateral_normal * velocity.linvel.dot(lateral_normal);
		let mass = get_mass(mass_properties);
		let friction_coefficient = 0.6;
		-lateral_velocity * 60.0 * mass * friction_coefficient
	}
}

fn get_mass(mass_properties: &AdditionalMassProperties) -> f32 {
    match mass_properties {
        AdditionalMassProperties::Mass(mass) => *mass,
        AdditionalMassProperties::MassProperties(properties) => properties.mass,
    }
}


#[cfg(tests)]
mod tests {
	use super::*;

}