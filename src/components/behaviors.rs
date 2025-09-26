use bevy::prelude::*;
use crate::components::physics::Position;
#[derive(Component, Clone, Copy)]
pub struct Goal {
    pub position: Position,
    pub radius: f32,
}


#[derive(Component)]
pub struct InCollision(pub bool);
