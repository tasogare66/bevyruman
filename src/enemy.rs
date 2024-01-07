use crate::components::*;
use bevy::{prelude::*, time::common_conditions::on_timer};
use std::time::Duration;

#[derive(Resource)]
pub struct EnemyCount {
    count: u32,
    max: u32,
}
impl Default for EnemyCount {
    fn default() -> Self {
        Self { count: 0, max: 10 }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyCount { ..default() }).add_systems(
            Update,
            enemy_spawn_system
                .run_if(on_timer(Duration::from_secs_f32(2. / 60.)))
                .in_set(GameSystemSet::PostUpdate),
        );
    }
}

fn enemy_spawn_system(mut commands: Commands, mut enemy_count: ResMut<EnemyCount>) {
    if enemy_count.count < enemy_count.max {
        let pos = Vec2::ZERO;
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb_u8(222, 184, 135),
                    custom_size: Some(Vec2::new(8., 8.)),
                    ..default()
                },
                transform: Transform {
                    translation: pos.extend(5.),
                    ..default()
                },
                ..default()
            })
            .insert(Enemy)
            .insert(PhysicalObj { ..default() });

        enemy_count.count += 1;
    }
}
