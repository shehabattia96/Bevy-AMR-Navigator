use bevy::prelude::*;
use crate::{components::physics::Position, systems::spawners::{spawn_amr, spawn_human}};

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Starting scene by spawning entities.");

    spawn_amr(&mut commands, &mut meshes, &mut materials, Position(Vec3::new(0.0, 0.0, 0.0)), Color::srgb(0.5, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
    spawn_human(&mut commands, &mut meshes, &mut materials, Position(Vec3::new(0.0, 0.0, 0.0)), Color::srgb(0.0, 0.2, 0.5), Vec2::new(1.0, 1.0));
}