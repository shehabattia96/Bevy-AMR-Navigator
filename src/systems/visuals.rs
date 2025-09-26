use bevy::prelude::*;

use crate::components::entity::Ground;

pub fn get_mouse_position_in_world_coordinates(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: &Single<&GlobalTransform, With<Ground>>,
    windows: &Query<&Window>,
) -> Option<Vec3>{
    let Ok(windows) = windows.single() else {
        return None;
    };

    let (camera, camera_transform) = *camera_query;

    let Some(cursor_position) = windows.cursor_position() else {
        return None ;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return None;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return None;
    };
    let point = ray.get_point(distance);

    return Some(point);
}

pub fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    // Displays a cursor on the ground plane at the cursor's position, just a neat UX thing.
    // from https://bevy.org/examples/3d-rendering/3d-viewport-to-world/
    let point = get_mouse_position_in_world_coordinates(camera_query, &ground, &windows);

    if point.is_none() {
        return;
    }

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(
        Isometry3d::new(
            point.unwrap() + ground.up() * 0.01,
            Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
        ),
        0.2,
        Color::WHITE,
    );
}
