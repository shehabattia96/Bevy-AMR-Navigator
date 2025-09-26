use bevy::prelude::*;

use crate::components::behaviors::Goal;
use crate::components::entity::{Ground, AMR};
use crate::components::physics::Position;
use crate::events::goal::GoalEvent;

pub fn on_mouse_click_broadcast_goal(
    mut goal_events: EventWriter<GoalEvent>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    let Ok(windows) = windows.single() else {
        return;
    };

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    if mouse_input.just_pressed(MouseButton::Left) {
        goal_events.write(GoalEvent(Goal {
            position: Position{0: Vec3::new(cursor_position.x, 0.0, cursor_position.y)},
            radius: 1.0
        }));
    }
}

pub fn goal_event_handler(mut goal_events: EventReader<GoalEvent>, mut amr_query: Query<&mut AMR>) {
    for goal_event in goal_events.read() {
        println!(
            "Goal event received at position: {:?}. Updating all AMR's!",
            goal_event.0.position.0
        );

        for mut amr in amr_query.iter_mut() {
            amr.1 = Some(goal_event.0.clone());
        }
    }
}
