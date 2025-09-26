use bevy::prelude::*;

use crate::components::{behaviors::Goal};

#[derive(Event)]
pub struct GoalEvent(pub Goal);