use bevy::prelude::*;

#[derive(Component)]
pub struct Position(Vec3);

#[derive(Component)]
pub struct Velocity(Vec3);

#[derive(Component)]
pub struct Acceleration(Vec3);