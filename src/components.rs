use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSystemSet {
    PreProcess,
    Update,
    UpdatePhysics,
    PostUpdate,
}

#[derive(Component)]
pub struct PhysicalObj {
    pub old_pos: Vec2,
    pub move_vec: Vec2,
}
impl Default for PhysicalObj {
    fn default() -> Self {
        Self {
            old_pos: Vec2::new(0., 0.),
            move_vec: Vec2::new(0., 0.),
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;
