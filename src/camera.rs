use bevy::prelude::*;

use crate::components::{MainCamera, Player};

pub fn update_camera_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<
        (&mut Transform, &mut OrthographicProjection),
        (With<MainCamera>, Without<Player>),
    >,
) {
    let Ok((mut transform, mut proj)) = camera_query.get_single_mut() else {
        return;
    };
    proj.scale = 1. / 4.;

    let Ok(player) = player_query.get_single() else {
        return;
    };

    let npos = transform
        .translation
        .xy()
        .lerp(player.translation.xy(), 0.08);
    transform.translation.x = npos.x;
    transform.translation.y = npos.y;
}
