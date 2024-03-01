use bevy::prelude::*;

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct GameLevel {
    positions: Vec<[f32; 3]>,
}
#[derive(Resource)]
pub struct GameLevelHandle(pub Handle<GameLevel>);
