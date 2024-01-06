use crate::components::{GameSystemSet, PhysicalObj, Player};
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
                player_keyboard_event_system.in_set(GameSystemSet::Update),
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

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut PhysicalObj, With<Player>>,
) {
    if let Ok(mut obj) = query.get_single_mut() {
        let mut mov = Vec2::new(0., 0.);
        mov.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };
        mov.y = if kb.pressed(KeyCode::Up) {
            1.
        } else if kb.pressed(KeyCode::Down) {
            -1.
        } else {
            0.
        };
        let mov = mov.normalize_or_zero();
        obj.move_vec += mov;
    }
}
