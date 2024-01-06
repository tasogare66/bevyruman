use bevy::prelude::*;
use components::{GameSystemSet, PhysicalObj, UniformVelocity};
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
        .configure_sets(
            Update,
            (
                GameSystemSet::Update.after(GameSystemSet::PreProcess),
                GameSystemSet::UpdatePhysics.after(GameSystemSet::Update),
                GameSystemSet::PostUpdate.after(GameSystemSet::UpdatePhysics),
            ),
        )
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup_system)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(
            Update,
            physical_obj_pre_proc_system.in_set(GameSystemSet::PreProcess),
        )
        .add_systems(
            Update,
            uniform_linear_motion_system.in_set(GameSystemSet::Update),
        )
        .add_systems(
            Update,
            physical_obj_do_verlet_system.in_set(GameSystemSet::UpdatePhysics),
        )
        .run();
}

fn setup_system(mut commands: Commands) {
    // camera
    commands.spawn(Camera2dBundle::default());
}

fn physical_obj_pre_proc_system(mut query: Query<&mut PhysicalObj>) {
    for mut obj in query.iter_mut() {
        obj.move_vec = Vec2::ZERO;
    }
}

fn physical_obj_do_verlet_system(mut query: Query<(&PhysicalObj, &mut Transform)>) {
    for (obj, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        *translation += obj.move_vec.extend(0.);
    }
}

// 等速直線運動,bullet等
fn uniform_linear_motion_system(
    time: Res<Time>,
    mut query: Query<(&UniformVelocity, &mut PhysicalObj)>,
) {
    for (v, mut obj) in query.iter_mut() {
        obj.move_vec = v.0 * time.delta_seconds();
    }
}
