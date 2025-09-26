use bevy::prelude::*;

use crate::components::physics::Position;

#[derive(Component)]
pub struct Goal {
    pub position: Position,
    pub radius:f32,
    pub is_reached: bool,
}

#[derive(Component)]
pub struct InCollision(bool);