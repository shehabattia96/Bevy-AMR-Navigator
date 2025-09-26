use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Position(pub Vec3);

#[derive(Component, Clone, Copy)]
pub struct Velocity(pub Vec3);

#[derive(Component, Clone, Copy)]
pub struct Acceleration(pub Vec3);