use bevy::prelude::*;
use crate::{components::physics::Position, systems::spawners::{spawn_amr, spawn_human}, components::entity::Ground};

fn setup_world(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,) {
    // From https://bevy.org/examples/3d-rendering/3d-viewport-to-world/

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Ground,
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Starting scene by spawning entities.");

    setup_world(&mut commands, &mut meshes, &mut materials,);

    let position = Position(Vec3::new(0.0, 0.0, 0.0));
    let size = Vec3::new(3.0, 2.5, 1.5);
    let position_off_ground = position.0 + Vec3::new(0.0, size[1] / 2.0, 0.0);
    spawn_amr(&mut commands, &mut meshes, &mut materials, Position(position_off_ground), Color::srgb(0.8, 0.2, 0.2), size);

    
    let position = Position(Vec3::new(2.0, 0.0, 2.0));
    let size =Vec2::new(0.3, 2.0);
    let position_off_ground = position.0 + Vec3::new(0.0, size[1] / 2.0, 0.0);
    spawn_human(&mut commands, &mut meshes, &mut materials, Position(position_off_ground), Color::srgb(0.2, 0.4, 0.8), size);

}