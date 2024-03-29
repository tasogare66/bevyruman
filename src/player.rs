use crate::{components::*, inputmng::InputMngBtn, AppState};
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Resource)]
struct PlayerState {
    alive: bool, // alive
}
impl Default for PlayerState {
    fn default() -> Self {
        Self { alive: false }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_systems(
                Update,
                (
                    player_input_move_event_system,
                    player_input_shot_event_system,
                )
                    .chain()
                    .in_set(GameSystemSet::Update)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                player_spawn_system
                    .in_set(GameSystemSet::PostUpdate)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

fn player_spawn_system(mut commands: Commands, mut player_state: ResMut<PlayerState>) {
    if !player_state.alive {
        let player_pos = Vec2::new(0., 0.);
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.95, 0.95, 0.95),
                    custom_size: Some(Vec2::new(8., 8.)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: player_pos.extend(10.),
                    ..Default::default()
                },
                ..default()
            })
            .insert(Player)
            .insert(PhysicalObj {
                old_pos: player_pos,
                ..default()
            })
            .insert(CollideCircle {
                radius: 3.,
                ..default()
            })
            .insert(Health::from_max(10.))
            .with_children(|parent| {
                parent.spawn(Weapon { ..default() }).insert(ForPlayer);
            });

        player_state.alive = true; //spawned
    }
}

fn player_input_move_event_system(
    input: Res<ButtonInput<InputMngBtn>>,
    time: Res<Time>,
    mut query: Query<&mut PhysicalObj, With<Player>>,
) {
    let Ok(mut obj) = query.get_single_mut() else {
        return;
    };

    //move
    let mut mov = Vec2::new(0., 0.);
    mov.x = if input.pressed(InputMngBtn::Left) {
        -1.
    } else if input.pressed(InputMngBtn::Right) {
        1.
    } else {
        0.
    };
    mov.y = if input.pressed(InputMngBtn::Up) {
        1.
    } else if input.pressed(InputMngBtn::Down) {
        -1.
    } else {
        0.
    };
    let mov = mov.normalize_or_zero();
    obj.move_vec += mov * time.delta_seconds() * 60.;
}

fn calc_screen_to_world_position(
    screen_pos: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Vec2 {
    let world_pos = camera
        .viewport_to_world(camera_transform, screen_pos)
        .map(|ray| ray.origin.truncate());
    match world_pos {
        Some(p) => p,
        None => Vec2::ZERO,
    }
}

fn player_input_shot_event_system(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<ButtonInput<InputMngBtn>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    query: Query<&Transform, With<Player>>,
    mut weapon_query: Query<&mut Weapon, With<ForPlayer>>,
) {
    let Ok(tf) = query.get_single() else {
        return;
    };
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };
    let Some(cur_pos) = window.cursor_position() else {
        return;
    };

    //shot
    if input.pressed(InputMngBtn::Shot) {
        let cur_world_pos = calc_screen_to_world_position(cur_pos, camera, camera_transform);
        let pos = tf.translation.xy();
        let Some(dir) = (cur_world_pos - pos).try_normalize() else {
            return;
        };
        let velocity = dir * 150.;

        let mut spawn_bullet = |offset: Vec2| {
            let bullet_pos = pos + offset;
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.95, 0.95, 0.95),
                        custom_size: Some(Vec2::new(8., 4.)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: bullet_pos.extend(10.),
                        rotation: Quat::from_rotation_z(dir.y.atan2(dir.x)), //angle
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(UniformVelocityBundle {
                    velocity: UniformVelocity(velocity),
                    physicalobj: PhysicalObj {
                        old_pos: bullet_pos,
                        ..default()
                    },
                    ..default()
                })
                .insert(Lifetime(Timer::from_seconds(1., TimerMode::Once)))
                .insert(DamageSource { ..default() })
                .insert(HitCircle { ..default() })
                .insert(FromPlayer);
        };
        for mut weapon in weapon_query.iter_mut() {
            weapon.repeat.tick(time.delta());
            if weapon.repeat.finished() {
                spawn_bullet(dir * 4.);
            }
        }
    }
}
