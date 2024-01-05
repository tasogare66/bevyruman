use bevy::prelude::*;
use player::PlayerPlugin;

mod components;
mod player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            68.0 / 225.0,
            36.0 / 255.0,
            52.0 / 255.0,
        )))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevyruma".into(),
                resolution: (1280f32, 720f32).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    // camera
    commands.spawn(Camera2dBundle::default());
}
