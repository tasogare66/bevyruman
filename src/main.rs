use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevyruma".into(),
                resolution: (1024f32, 720f32).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run();
}
