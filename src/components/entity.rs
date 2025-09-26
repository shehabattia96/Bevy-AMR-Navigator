use bevy::prelude::*;

use crate::components::{behaviors::InCollision, physics::{Acceleration, Position, Velocity}};

#[derive(Component)]
pub struct CollidableEntity {
    pub id: u128,
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub bounding_box: Vec3,
    pub in_collision: InCollision,
}


#[derive(Component,)]
pub struct AMR(pub CollidableEntity);

#[derive(Component)]
pub struct Human(pub CollidableEntity);

#[derive(Component)]
pub struct Obstacle(pub CollidableEntity);


#[derive(Component)]
pub struct Ground;