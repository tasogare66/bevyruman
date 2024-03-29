use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSystemSet {
    PreProcess,
    Update,
    UpdatePhysics,
    PostPhysics,
    PostUpdate,
}

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct PhysicalObj {
    pub inv_mass: f32, //0.0fだと質量無限
    pub old_pos: Vec2,
    pub move_vec: Vec2,
    pub old_move_vec: Vec2,
    pub force: Vec2,
    pub velocity: Vec2,
    pub collision_count: u32,
}
impl Default for PhysicalObj {
    fn default() -> Self {
        Self {
            inv_mass: 1.,
            old_pos: Vec2::new(0., 0.),
            move_vec: Vec2::new(0., 0.),
            old_move_vec: Vec2::new(0., 0.),
            force: Vec2::new(0., 0.),
            velocity: Vec2::new(0., 0.),
            collision_count: 0,
        }
    }
}

// 衝突する,円
#[derive(Component)]
pub struct CollideCircle {
    pub radius: f32,
}
impl Default for CollideCircle {
    fn default() -> Self {
        Self { radius: 4. }
    }
}

// 接触判定する,円
#[derive(Component)]
pub struct HitCircle {
    pub radius: f32,
}
impl Default for HitCircle {
    fn default() -> Self {
        Self { radius: 3. }
    }
}

#[derive(Component)]
pub struct Lifetime(pub Timer);

// 体力,0でdespawn
#[derive(Component)]
pub struct Health {
    pub hp: f32,
    pub max: f32,
}
impl Health {
    pub fn from_max(max: f32) -> Self {
        assert!(max > 0.);
        Self { hp: max, max: max }
    }
    pub fn get_ratio(&self) -> f32 {
        self.hp / self.max
    }
}

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
pub struct Enemy;

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct ForPlayer;

#[derive(Component)]
pub struct DamageSource {
    pub damage: f32,
}
impl Default for DamageSource {
    fn default() -> Self {
        Self { damage: 1. }
    }
}

#[derive(Component)]
pub struct Weapon {
    pub repeat: Timer,
}
impl Default for Weapon {
    fn default() -> Self {
        Self {
            repeat: Timer::from_seconds(1. / 60. * 4., TimerMode::Repeating),
        }
    }
}
