use bevy::ecs::system::Commands;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 5.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(10.0))
        .insert(Restitution::coefficient(1.))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 200.0, 0.0)));
}