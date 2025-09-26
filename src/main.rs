use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

mod components;

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
        .run();
}
