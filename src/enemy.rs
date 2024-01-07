use crate::components::*;
use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::Rng;
use std::{f32::consts::PI, time::Duration};

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

fn random_circle_base(r0: f32, ed_r: f32, half_central_ang: f32) -> Vec2 {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(r0..1.).sqrt() * ed_r;
    let theta = rng.gen_range(-half_central_ang..half_central_ang);
    Vec2::new(r * theta.cos(), r * theta.sin())
}

fn random_circle(st_r: f32, ed_r: f32) -> Vec2 {
    let r0 = st_r / ed_r;
    random_circle_base(r0, ed_r, PI)
}

fn enemy_spawn_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    q_player: Query<&Transform, With<Player>>,
) {
    let Ok(pl_tf) = q_player.get_single() else {
        return;
    };
    if enemy_count.count < enemy_count.max {
        let pos = random_circle(100., 200.) + pl_tf.translation.xy();
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