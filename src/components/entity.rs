use bevy::prelude::*;

use crate::components::{
    behaviors::{Goal, InCollision},
    physics::{Acceleration, Position, Velocity},
};

#[derive(Component)]
pub struct DynamicEntity {
    pub id: u128,
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub bounding_box: Vec3,
    pub in_collision: InCollision,
    pub max_velocity: f32,
}

#[derive(Component)]
pub struct AMR(pub DynamicEntity, pub Option<Goal>);

#[derive(Component)]
pub struct Human(pub DynamicEntity);

#[derive(Component)]
pub struct Obstacle(pub DynamicEntity);

#[derive(Component)]
pub struct Ground;
