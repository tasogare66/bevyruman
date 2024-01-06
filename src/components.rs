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
pub struct Lifetime(pub Timer);

// 等速直線運動
#[derive(Component)]
pub struct UniformVelocity(pub Vec2);

#[derive(Bundle)]
pub struct UniformVelocityBundle {
    pub velocity: UniformVelocity,
    pub physicalobj: PhysicalObj,
}
impl Default for UniformVelocityBundle {
    fn default() -> Self {
        Self {
            velocity: UniformVelocity(Vec2::new(0., 0.)),
            physicalobj: Default::default(),
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct Enemy;
