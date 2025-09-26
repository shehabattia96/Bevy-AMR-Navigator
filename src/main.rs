use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

mod components;
mod events;
mod systems;

use events::goal::GoalEvent;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "AMR Navigation".into(),
                        ..default()
                    }),
                    ..default()
                }),
                FrameTimeDiagnosticsPlugin::default(),
                LogDiagnosticsPlugin::default(),
            )
        )
        .add_event::<GoalEvent>()
        .add_systems(Startup, (
            systems::startup::startup,
        ))
        .add_systems(Update, (
            systems::goal_event_handler::goal_event_handler,
            systems::goal_event_handler::on_mouse_click_broadcast_goal,
            systems::visuals::draw_cursor,
            
            systems::amr_controller::amr_move_to_goal

        ))
        .run();
}
