use crate::components::behaviors::Goal;
use crate::components::entity::AMR;
use crate::components::entity::Human;

use bevy::prelude::*;

pub fn amr_move_to_goal(
    mut amr_query: Query<(&mut AMR, &mut Transform)>,
    mut other_objects_query: Query<&Human>,
    time: Res<Time>,
) {
    for (mut amr, mut transform) in amr_query.iter_mut() {
        if amr.1.is_none() {
            // if there is no goal, we don't want to move
            return;
        }

        println!(
            "AMR {:?} is moving to goal: {:?}, position: {:?}",
            amr.0.id,
            amr.1.unwrap().position.0,
            amr.0.position.0
        );

        let mut vector_to_goal = get_vector_to_goal(&amr);

        vector_to_goal.y = 0.0;

        let target_direction = vector_to_goal.normalize();

        amr.0.velocity.0 = target_direction * amr.0.max_velocity;

        let distance = vector_to_goal.length();

        let is_reached = is_reached_goal(distance, &amr.1.unwrap());

        if is_reached {
            amr.1 = None;
            println!("AMR {:?} has reached the goal", amr.0.id);
            return;
        }

        let cloest_target_position = detect_closest_target_in_range(
            &amr,
            &mut other_objects_query,
            amr.0.bounding_box.max_element() + 0.2, // TODO: magic number
        );
        println!(
            "cloest_target_position {:?}cloest_target_positioncloest_target_positioncloest_target_positioncloest_target_positioncloest_target_positioncloest_target_position",
            cloest_target_position
        );

        let max_vel = amr.0.max_velocity;
        if cloest_target_position != Vec3::ZERO {
            let avoidance_vector = get_avoidance_vector(
                amr.0.position.0,
                cloest_target_position
            );

            let obstacle_direction = (cloest_target_position - amr.0.position.0).normalize();

            let obstacle_is_in_front = obstacle_direction.dot(target_direction) > 0.0;

            // Skip applying the avoidance vector if the obstacle is behind us, otherwise we get stuck adjacent to the object we're trying to avoid..
            if obstacle_is_in_front {
                amr.0.velocity.0 = amr.0.velocity.0 * 0.3 + 0.7 * avoidance_vector * max_vel; // TODO: magic numbers
            }
        }

        // Update position based on velocity
        let velocity = amr.0.velocity.0;
        let delta_time = time.delta_secs();

        // Update the position and transform
        amr.0.position.0 += velocity * delta_time;
        transform.translation = amr.0.position.0;
    }
}

pub fn detect_closest_target_in_range(
    amr: &AMR,
    other_objects_query: &mut Query<(&Human)>,
    range: f32,
) -> Vec3 {
    let mut least_distance = f32::MAX;
    let mut closest_target = Vec3::ZERO;

    for (other_objects) in other_objects_query.iter_mut() {
        if other_objects.0.id == amr.0.id {
            continue;
        }
        let distance = amr.0.position.0.distance(other_objects.0.position.0);

        if distance <= range {
            if distance < least_distance {
                least_distance = distance;
                closest_target = other_objects.0.position.0;
            }
        }
    }
    return closest_target;
}

fn get_avoidance_vector(
    amr_pos: Vec3,
    other_pos: Vec3
) -> Vec3 {
    // Calculate a simple avoidance perpendicular to the target:
    let to_obstacle = (other_pos - amr_pos).normalize();
    let distance_to_other = amr_pos.distance(other_pos);

    // Perpendicular to the obstacle
    let avoidance_dir = Vec3::new(-to_obstacle.z, 0.0, to_obstacle.x);

    return avoidance_dir;
}

fn get_vector_to_goal(amr: &AMR) -> Vec3 {
    if amr.1.is_none() {
        return Vec3::ZERO;
    }
    let goal = amr.1.unwrap();
    let amr_position = amr.0.position.0;
    let goal_position = goal.position.0;

    let vector = goal_position - amr_position;

    return vector;
}

fn is_reached_goal(current_distance: f32, goal: &Goal) -> bool {
    // TODO: do a bounding box check:
    if current_distance <= goal.radius {
        return true;
    }

    return false;
}
