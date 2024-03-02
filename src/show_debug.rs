use crate::{components::*, GameConfig};
use bevy::prelude::*;

pub struct ShowDebugPlugin;

impl Plugin for ShowDebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_systems(
                Update,
                (show_bg_gizmo_system, show_colli_gizmo_system).in_set(GameSystemSet::PostUpdate),
            );
        }
    }
}

fn show_bg_gizmo_system(mut gizmos: Gizmos) {
    gizmos.line_2d(Vec2::X * 10., Vec2::ZERO, Color::RED);
    gizmos.line_2d(Vec2::Y * 10., Vec2::ZERO, Color::GREEN);
    // The circles have 32 line-segments by default.
    gizmos.circle_2d(Vec2::ZERO, 100., Color::BLACK);
    // You may want to increase this for larger circles.
    gizmos
        .circle_2d(Vec2::ZERO, 170., Color::WHITE)
        .segments(64);
}

fn show_colli_gizmo_system(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &CollideCircle)>,
    game_config: Res<GameConfig>,
) {
    // show collision
    if cfg!(debug_assertions) && game_config.dbg_show_collision {
        for (transform, colli) in query.iter() {
            let pos = transform.translation.xy();
            gizmos
                .circle_2d(pos, colli.radius, Color::ORANGE)
                .segments(16);
        }
    }
}
