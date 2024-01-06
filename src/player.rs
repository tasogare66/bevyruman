use crate::components::*;
use bevy::prelude::*;

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
                player_input_event_system.in_set(GameSystemSet::Update),
            )
            .add_systems(
                Update,
                player_spawn_system.in_set(GameSystemSet::PostUpdate),
            );
    }
}

fn player_spawn_system(mut commands: Commands, mut player_state: ResMut<PlayerState>) {
    if !player_state.alive {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.95, 0.95, 0.95),
                    custom_size: Some(Vec2::new(8., 8.)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0., 0., 10.),
                    ..Default::default()
                },
                ..default()
            })
            .insert(Player)
            .insert(PhysicalObj { ..default() });

        player_state.alive = true; //spawned
    }
}

fn player_input_event_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut PhysicalObj), With<Player>>,
) {
    if let Ok((tf, mut obj)) = query.get_single_mut() {
        //move
        let mut mov = Vec2::new(0., 0.);
        mov.x = if kb.pressed(KeyCode::Left) || kb.pressed(KeyCode::A) {
            -1.
        } else if kb.pressed(KeyCode::Right) || kb.pressed(KeyCode::D) {
            1.
        } else {
            0.
        };
        mov.y = if kb.pressed(KeyCode::Up) || kb.pressed(KeyCode::W) {
            1.
        } else if kb.pressed(KeyCode::Down) || kb.pressed(KeyCode::S) {
            -1.
        } else {
            0.
        };
        let mov = mov.normalize_or_zero();
        obj.move_vec += mov * time.delta_seconds() * 60.;

        //shot
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (tf.translation.x, tf.translation.y);
            let mut spawn_bullet = |x_offset: f32| {
                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.95, 0.95, 0.95),
                            custom_size: Some(Vec2::new(4., 8.)),
                            ..Default::default()
                        },
                        transform: Transform {
                            translation: Vec3::new(x, y, 10.),
                            ..Default::default()
                        },
                        ..default()
                    })
                    .insert(UniformVelocityBundle {
                        velocity: UniformVelocity(Vec2::new(0., 150.)),
                        ..default()
                    })
                    .insert(Lifetime(Timer::from_seconds(1., TimerMode::Once)))
                    .insert(FromPlayer);
            };
            spawn_bullet(0.);
        }
    }
}
