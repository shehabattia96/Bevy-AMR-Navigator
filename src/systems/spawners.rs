use bevy::prelude::*;
use bevy::asset::uuid::Uuid;
use crate::components::{behaviors::{InCollision}, entity::{CollidableEntity, Human, AMR}, physics::{Acceleration, Position, Velocity}};

pub fn spawn_amr(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Position,
    material: Color,
    size: Vec3
) {
    // we'll always spawn as a cuboid, so we only need to pass in position, size and color
    commands.spawn((
        AMR(CollidableEntity {
            id: Uuid::new_v4().as_u128(),
            position: position,
            bounding_box: size,
            velocity: Velocity(Vec3::new(0.0, 0.0, 0.0)),
            acceleration: Acceleration(Vec3::new(0.0, 0.0, 0.0)),
            in_collision: InCollision(false),
        }),
        Mesh3d(meshes.add(Cuboid::new(size[0], size[1], size[2]))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(material))),
        Transform::from_translation(position.0),
    ));
}
pub fn spawn_human(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Position,
    material: Color,
    size: Vec2
) {
    // size is radius, height
    commands.spawn((
        Human(CollidableEntity {
            id: Uuid::new_v4().as_u128(),
            position: position,
            bounding_box: Vec3::new(size[0], size[0], size[1]),
            velocity: Velocity(Vec3::new(0.0, 0.0, 0.0)),
            acceleration: Acceleration(Vec3::new(0.0, 0.0, 0.0)),
            in_collision: InCollision(false),
        }),
        Mesh3d(meshes.add(Capsule3d::new(size[0], size[1]))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(material))),
        Transform::from_translation(position.0),
    ));
}
