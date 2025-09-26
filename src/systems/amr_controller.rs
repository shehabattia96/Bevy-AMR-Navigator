use crate::components::behaviors::Goal;
use crate::components::{entity::AMR};
use bevy::prelude::*;



pub fn amr_move_to_goal(mut amr_query: Query<(&mut AMR, &mut Transform)>, time: Res<Time>) {
    for (mut amr, mut transform) in amr_query.iter_mut() {

        if  amr.1.is_none()  {
            // if there is no goal, we don't want to move
            return;
        }

        println!("AMR {:?} is moving to goal: {:?}, position: {:?}", amr.0.id, amr.1.unwrap().position.0, amr.0.position.0 );

        let mut vector_to_goal = get_vector_to_goal(&amr);

        vector_to_goal.y = 0.0;

        amr.0.velocity.0 =  vector_to_goal.normalize()* amr.0.max_velocity;

        let distance = vector_to_goal.length();

        let is_reached = is_reached_goal(distance, &amr.1.unwrap());

        if is_reached {
            amr.1 = None;
            println!("AMR {:?} has reached the goal", amr.0.id);
            return
        }

        // Update position based on velocity
        let velocity = amr.0.velocity.0;
        let delta_time = time.delta_secs();

        // Update the position and transform
        amr.0.position.0 += velocity * delta_time;
        transform.translation = amr.0.position.0;
    }
}


fn get_vector_to_goal(amr: &AMR) -> Vec3 {
    if  amr.1.is_none()  {
        return Vec3::ZERO;
    }
    let goal = amr.1.unwrap();
    let amr_position = amr.0.position.0;
    let goal_position =goal.position.0;

    let vector = goal_position - amr_position;

    return vector
}

fn is_reached_goal(current_distance: f32, goal:&Goal) -> bool{
    // TODO: do a bounding box check:
    if current_distance <= goal.radius {
        return true;
    }

    return false;
}