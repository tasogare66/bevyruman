use crate::components::*;
use crate::resources::*;
use bevy::{prelude::*, time::common_conditions::on_timer, window::PresentMode};
use dw_gui::DwGuiPlugin;
use enemy::{EnemyCount, EnemyPlugin};
use moonshine_save::prelude::*;
use player::PlayerPlugin;
use ron_asset::RonAssetPlugin;
use show_debug::ShowDebugPlugin;
use show_fps::ShowFpsPlugin;
use sparse_grid::{Aabb, SparseGrid2d};
use std::path::Path;
use std::time::Duration;
use ui_game::UiGamePlugin;

mod camera;
mod components;
mod dw_gui;
mod enemy;
mod inputmng;
mod levelup;
mod player;
mod resources;
mod ron_asset;
mod shop;
mod show_debug;
mod show_fps;
pub mod sparse_grid;
mod title;
mod ui_game;

const TILE_SIZE: usize = 10;
const SAVE_CONFIG_PATH: &str = "ram/config.ron";

#[derive(Resource)]
struct PhysicsResource {
    pub prev_dt: f32, //1frame前のdt
}
impl Default for PhysicsResource {
    fn default() -> Self {
        Self {
            prev_dt: 1.0 / 60.0,
        }
    }
}

#[derive(Debug, Resource)]
pub struct SHM {
    sg2: SparseGrid2d<TILE_SIZE>,
}

#[derive(Resource)]
pub struct GameFonts {
    cmn: Handle<Font>,
}

#[derive(Resource)]
pub struct GameTextures {
    spr0_tex: Handle<Image>,
    spr0_layout: Handle<TextureAtlasLayout>,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GameConfig {
    dbg_show_collision: bool,
    dbg_least_time: bool, //ゲームすぐに終了
    float: f32,
}
impl Default for GameConfig {
    fn default() -> Self {
        Self {
            dbg_show_collision: false,
            dbg_least_time: false,
            float: 0.,
        }
    }
}
#[derive(Bundle)]
struct GameConfigBundle {
    game_config: GameConfig,
    save: Save,
}

/// A resource which is used to invoke the save system.
#[derive(Resource)]
struct SaveConfigRequest;
impl SaveIntoFileRequest for SaveConfigRequest {
    fn path(&self) -> &Path {
        SAVE_CONFIG_PATH.as_ref()
    }
}

/// A resource which is used to invoke the load system.
#[derive(Resource)]
struct LoadConfigRequest;
impl LoadFromFileRequest for LoadConfigRequest {
    fn path(&self) -> &Path {
        SAVE_CONFIG_PATH.as_ref()
    }
}

// Gameシーケンス
#[derive(Resource)]
pub struct GameSequence {
    started: bool,
    wave_no: u32,
}
impl Default for GameSequence {
    fn default() -> Self {
        Self {
            started: false,
            wave_no: 0,
        }
    }
}

// waveの状態
#[derive(Resource)]
pub struct WaveStatus {
    timer: Timer,
}
impl Default for WaveStatus {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(60.0, TimerMode::Once),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Title,
    InGame,
    LevelUp,
    Shop,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            68.0 / 225.0,
            36.0 / 255.0,
            52.0 / 255.0,
        )))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevyruman".into(),
                        resolution: (1280f32, 720f32).into(),
                        present_mode: PresentMode::AutoNoVsync, //fps見るため,vsync off
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()), //texture別に設定したいけど,やり方分からない
        )
        .add_plugins(RonAssetPlugin::<GameLevel>::new(&["level.ron"]))
        //save load
        .add_plugins(SavePlugin)
        .register_type::<GameConfig>()
        .add_systems(
            PreUpdate,
            //save_default().into_file_on_request::<SaveConfigRequest>(),
            save::<With<GameConfig>>().into_file_on_request::<SaveConfigRequest>(),
        )
        .add_systems(PreUpdate, load_from_file_on_request::<LoadConfigRequest>())
        .configure_sets(
            Update,
            (
                GameSystemSet::Update.after(GameSystemSet::PreProcess),
                GameSystemSet::UpdatePhysics.after(GameSystemSet::Update),
                GameSystemSet::PostPhysics.after(GameSystemSet::UpdatePhysics),
                GameSystemSet::PostUpdate.after(GameSystemSet::PostPhysics),
            ),
        )
        .insert_resource(PhysicsResource { ..default() })
        .insert_resource(SHM {
            sg2: SparseGrid2d::<TILE_SIZE>::default(),
        })
        .insert_resource(GameSequence { ..default() })
        .insert_resource(WaveStatus { ..default() })
        .add_plugins((ShowDebugPlugin, ShowFpsPlugin, DwGuiPlugin))
        .add_systems(PreStartup, pre_startup_setup_system)
        .add_systems(Startup, inputmng::startup_input_mng_system)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, inputmng::update_input_mng_system)
        .init_state::<AppState>()
        //Title
        .add_systems(OnEnter(AppState::Title), title::setup_title)
        .add_systems(
            Update,
            title::title_system.run_if(in_state(AppState::Title)),
        )
        .add_systems(
            OnExit(AppState::Title),
            (title::cleanup_title, setup_game_sequence_system),
        )
        //LevelUp
        .add_systems(OnEnter(AppState::LevelUp), levelup::setup_levelup)
        .add_systems(
            Update,
            levelup::levelup_system.run_if(in_state(AppState::LevelUp)),
        )
        .add_systems(
            OnExit(AppState::LevelUp),
            (levelup::cleanup_levelup, ui_game::cleanup_ui_game_system),
        )
        //Shop
        .add_systems(OnEnter(AppState::Shop), shop::setup_shop)
        .add_systems(Update, shop::shop_system.run_if(in_state(AppState::Shop)))
        .add_systems(OnExit(AppState::Shop), shop::cleanup_shop)
        //InGame
        .add_plugins((PlayerPlugin, EnemyPlugin))
        .add_plugins((UiGamePlugin,))
        .add_systems(
            OnEnter(AppState::InGame),
            (
                setup_in_game_system,
                ui_game::setup_ui_game,              //ui作る
                ui_game::update_ui_game_wave_system, //wave表示更新
            )
                .chain(),
        )
        .add_systems(OnExit(AppState::InGame), cleanup_in_game_system)
        .add_systems(
            Update,
            (physical_obj_pre_proc_system, shm_pre_proc_system)
                .in_set(GameSystemSet::PreProcess)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            uniform_linear_motion_system
                .in_set(GameSystemSet::Update)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (
                bullet_vs_enemy_system,
                //collision_detection_system,
                collision_detection_shm_system,
                //(move_ball_system, shm_pre_proc_system).chain(),
            )
                .in_set(GameSystemSet::UpdatePhysics)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (
                physical_obj_do_verlet_system,
                //do_constraints_system
            )
                .in_set(GameSystemSet::PostPhysics)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (update_entity_existence_system, update_wave_system)
                .in_set(GameSystemSet::PostUpdate)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            PostUpdate,
            camera::update_camera_system
                .run_if(on_timer(Duration::from_secs_f32(1. / 60.)))
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}

fn pre_startup_setup_system(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));
    // add font resource
    let game_fonts = GameFonts {
        cmn: asset_server.load("MPLUS1Code-Regular.ttf"),
    };
    commands.insert_resource(game_fonts);
    // sprite
    let texture_handle = asset_server.load("sprites_000.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(8., 8.), 16, 32, None, None);
    let layout_handle = texture_atlases.add(layout);
    let game_texture = GameTextures {
        spr0_tex: texture_handle,
        spr0_layout: layout_handle,
    };
    commands.insert_resource(game_texture);

    let level = GameLevelHandle(asset_server.load("game.level.ron"));
    commands.insert_resource(level);

    commands.spawn((GameConfigBundle {
        game_config: GameConfig { ..default() },
        save: Save,
    },));
    commands.insert_resource(crate::LoadConfigRequest);
}

fn physical_obj_pre_proc_system(mut query: Query<(&Transform, &mut PhysicalObj)>) {
    for (transform, mut obj) in query.iter_mut() {
        obj.move_vec = Vec2::ZERO;
        obj.old_move_vec = Vec2::ZERO;
        obj.force = Vec2::ZERO;
        obj.velocity = transform.translation.xy() - obj.old_pos;
        obj.collision_count = 0;
    }
}

fn shm_pre_proc_system(mut shm: ResMut<SHM>, query: Query<(Entity, &Transform, &CollideCircle)>) {
    // clearして、登録しなおす
    shm.sg2.soft_clear();
    for (entity, transform, colli) in query.iter() {
        shm.sg2.insert_aabb(
            Aabb::from_circle(transform.translation.xy(), colli.radius),
            entity,
        );
    }
}

#[allow(dead_code)]
fn move_ball_system(mut query: Query<(&mut PhysicalObj, &mut Transform)>) {
    for (mut obj, mut transform) in query.iter_mut() {
        let pos_next = transform.translation.xy() + obj.move_vec;
        let pos_old = obj.old_pos + obj.move_vec;
        let vel = pos_next - pos_old;
        let max_speed = if obj.collision_count >= 2 {
            4. * (1.0 / obj.collision_count as f32)
        } else {
            100.
        };
        let len = vel.length();
        let rcp = len.recip();
        let spd = len.min(max_speed);

        let pos_next = pos_next
            + if rcp.is_finite() && rcp > 0.0 {
                vel * rcp * spd //normalize * spd
            } else {
                Vec2::ZERO
            };
        let pos_old = pos_old + vel;

        let translation = &mut transform.translation;
        *translation = pos_next.extend(translation.z);
        obj.old_pos = pos_old;
    }
}

#[allow(dead_code)]
fn do_constraints_system(
    #[allow(unused_mut)] mut query: Query<(
        Entity,
        &mut Transform,
        &CollideCircle,
        &mut PhysicalObj,
    )>,
    _shm: Res<SHM>,
) {
    #[allow(unused_unsafe)]
    unsafe {
        // for (e0, mut tf0, colli0, mut obj0) in query.iter_unsafe() {
        //     for e1 in _shm
        //         .sg2
        //         .query_aabb(Aabb::from_circle(tf0.translation.xy(), colli0.radius))
        //     {
        let mut iter = query.iter_combinations_mut();
        while let Some([(_e0, mut tf0, colli0, mut obj0), (_e1, mut tf1, colli1, mut obj1)]) =
            iter.fetch_next()
        {
            // if e0 > e1 {
            //     continue;
            // }
            // if let Ok((_, mut tf1, colli1, mut obj1)) = query.get_unchecked(e1) {
            // do something with the components
            let diff = tf1.translation.xy() - tf0.translation.xy();
            let dist = diff.length();
            let depth = colli0.radius + colli1.radius - dist;
            if dist > 0. && depth > 0. {
                // d==0: same particle
                let fac = 1. / dist * depth * 0.5;
                let mv = diff * fac;
                tf0.translation += mv.extend(0.);
                obj0.old_pos += mv;
                obj0.collision_count += 1;
                tf1.translation -= mv.extend(0.);
                obj1.collision_count += 1;
                obj1.old_pos -= mv;
            }
            // }
        }
    }
}

#[allow(dead_code)]
fn collision_detection_system(mut query: Query<(&Transform, &CollideCircle, &mut PhysicalObj)>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(tf0, colli0, mut obj0), (tf1, colli1, mut obj1)]) = iter.fetch_next() {
        let diff = tf1.translation.xy() - tf0.translation.xy();
        let d = diff.length();
        let target = colli0.radius + colli1.radius;
        if d > 0. && d <= target {
            // d==0: same particle
            let inv_mass0 = obj0.inv_mass;
            let inv_mass1 = obj1.inv_mass;
            let together_inv_mass = obj0.inv_mass + obj1.inv_mass;
            let imr0 = obj0.inv_mass / together_inv_mass;
            let imr1 = obj1.inv_mass / together_inv_mass;
            let factor = (d - target) / d;
            obj0.move_vec += diff * factor * imr0;
            obj1.move_vec -= diff * factor * imr1;
            // preserve impulse
            let ebounce = 0.5; //const_param::BOUNCE;
            let n = diff / d;
            let impulse_j =
                (1.0 + ebounce) * (obj0.velocity - obj1.velocity).dot(n) / together_inv_mass;
            // p1,apply impulse
            obj0.old_move_vec += n * (impulse_j * inv_mass0);
            obj0.collision_count += 1;
            //p1->m_hit_mask.set(p2->m_colli_attr);
            // p2,apply impulse
            obj1.old_move_vec -= n * (impulse_j * inv_mass1);
            obj0.collision_count += 1;
            //p2->m_hit_mask.set(p1->m_colli_attr);
        }
    }
}

fn collision_detection_shm_system(
    #[allow(unused_mut)] mut query: Query<(Entity, &Transform, &CollideCircle, &mut PhysicalObj)>,
    shm: Res<SHM>,
) {
    unsafe {
        for (e0, tf0, colli0, mut obj0) in query.iter_unsafe() {
            for e1 in shm
                .sg2
                .query_aabb(Aabb::from_circle(tf0.translation.xy(), colli0.radius))
            {
                if e0 > e1 {
                    continue;
                }
                if let Ok((_, tf1, colli1, mut obj1)) = query.get_unchecked(e1) {
                    // do something with the components
                    let diff = tf1.translation.xy() - tf0.translation.xy();
                    let d = diff.length();
                    let target = colli0.radius + colli1.radius;
                    if d > 0. && d <= target {
                        // d==0: same particle
                        let inv_mass0 = obj0.inv_mass;
                        let inv_mass1 = obj1.inv_mass;
                        let together_inv_mass = obj0.inv_mass + obj1.inv_mass;
                        let imr0 = obj0.inv_mass / together_inv_mass;
                        let imr1 = obj1.inv_mass / together_inv_mass;
                        let factor = (d - target) / d;
                        obj0.move_vec += diff * factor * imr0;
                        obj1.move_vec -= diff * factor * imr1;
                        // preserve impulse
                        let ebounce = 0.5; //const_param::BOUNCE;
                        let n = diff / d;
                        let impulse_j = (1.0 + ebounce) * (obj0.velocity - obj1.velocity).dot(n)
                            / together_inv_mass;
                        // p1,apply impulse
                        obj0.old_move_vec += n * (impulse_j * inv_mass0);
                        obj0.collision_count += 1;
                        //p1->m_hit_mask.set(p2->m_colli_attr);
                        // p2,apply impulse
                        obj1.old_move_vec -= n * (impulse_j * inv_mass1);
                        obj1.collision_count += 1;
                        //p2->m_hit_mask.set(p1->m_colli_attr);
                    }
                }
            }
        }
    }
}

fn physical_obj_do_verlet_system(
    time: Res<Time>,
    mut physics_resource: ResMut<PhysicsResource>,
    mut query: Query<(Entity, &mut PhysicalObj, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    if dt <= 0. {
        return;
    };
    let inv_prev_dt = 1. / physics_resource.prev_dt;
    let damping = 0.4;
    let decel = f32::powf(damping, dt);
    for (_entity, mut obj, mut transform) in query.iter_mut() {
        let mov_vec = obj.move_vec
            * if obj.collision_count >= 2 {
                (obj.collision_count as f32).recip() //FIXME:適当な対応
            } else {
                1.
            };
        let pos = transform.translation.xy() + mov_vec;
        let mut tmp = obj.old_pos + mov_vec;
        tmp = tmp + obj.old_move_vec; //change velocity

        // do verlet
        let vel = (pos - tmp) * inv_prev_dt;
        let inv_mass_dt = obj.inv_mass * dt;
        let vel = vel + obj.force * inv_mass_dt;
        let vel = vel * decel; //damping

        let tmp = pos + vel * dt;

        // set_position
        let translation = &mut transform.translation;
        *translation = tmp.extend(translation.z);
        obj.old_pos = pos;
        // set_velocity
        obj.velocity = vel;
    }
    physics_resource.prev_dt = dt;
}

fn intersect_circle_vs_circle(c0: Vec2, r0: f32, c1: Vec2, r1: f32) -> bool {
    let diff = c1 - c0;
    let sqr_d = diff.length_squared();
    let target = r0 + r1;
    sqr_d <= target * target
}

fn bullet_vs_enemy_system(
    mut bullet_query: Query<(Entity, &Transform, &HitCircle, &mut DamageSource), With<FromPlayer>>,
    mut ene_query: Query<(Entity, &Transform, &CollideCircle, &mut Health), With<Enemy>>,
    shm: Res<SHM>,
) {
    for (_, tf0, hit0, mut dmg0) in bullet_query.iter_mut() {
        //
        for e1 in shm
            .sg2
            .query_aabb(Aabb::from_circle(tf0.translation.xy(), hit0.radius))
        {
            if let Ok((_, tf1, colli1, mut health1)) = ene_query.get_mut(e1) {
                if dmg0.damage <= 0. {
                    break;
                }
                if health1.0 <= 0. {
                    continue;
                }
                if intersect_circle_vs_circle(
                    tf0.translation.xy(),
                    hit0.radius,
                    tf1.translation.xy(),
                    colli1.radius,
                ) {
                    let health = health1.0;
                    health1.0 -= dmg0.damage;
                    dmg0.damage -= health;
                }
            }
        }
    }
}

// 等速直線運動,bullet等
fn uniform_linear_motion_system(
    time: Res<Time>,
    mut query: Query<(&UniformVelocity, &mut PhysicalObj)>,
) {
    for (v, mut obj) in query.iter_mut() {
        obj.move_vec = v.0 * time.delta_seconds();
    }
}

fn update_entity_existence_system(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_count: ResMut<EnemyCount>,
    mut query: Query<(
        Entity,
        Option<&mut Lifetime>,
        Option<&Health>,
        Option<&DamageSource>,
        Option<&Enemy>,
    )>,
) {
    for (entity, timer, health, dmg, enemy) in query.iter_mut() {
        // 生存時間
        if let Some(mut timer) = timer {
            timer.0.tick(time.delta());
            if timer.0.finished() {
                commands.entity(entity).despawn();
                if enemy.is_some() {
                    enemy_count.count -= 1;
                }
                continue;
            }
        }
        // 体力
        if let Some(health) = health {
            if health.0 <= 0. {
                commands.entity(entity).despawn();
                if enemy.is_some() {
                    enemy_count.count -= 1;
                }
                continue;
            }
        }
        // damage
        if let Some(dmg) = dmg {
            if dmg.damage <= 0. {
                commands.entity(entity).despawn();
                if enemy.is_some() {
                    enemy_count.count -= 1;
                }
                continue;
            }
        }
    }
}

// GameSequence初期化処理
fn setup_game_sequence_system(mut game_sequence: ResMut<GameSequence>) {
    // clear
    *game_sequence = GameSequence { ..default() };
}

// InGame初期化処理
fn setup_in_game_system(
    game_config: Query<&GameConfig>,
    mut game_sequence: ResMut<GameSequence>,
    mut wave_status: ResMut<WaveStatus>,
) {
    // next wave
    if game_sequence.started {
        game_sequence.wave_no += 1;
    } else {
        game_sequence.started = true;
    }
    // clear
    *wave_status = WaveStatus { ..default() };
    // for debug,time短い設定
    if cfg!(debug_assertions) && game_config.get_single().unwrap().dbg_least_time {
        wave_status.timer = Timer::from_seconds(5.0, TimerMode::Once);
    }
}

// InGame終了処理
fn cleanup_in_game_system() {
    //
}

fn update_wave_system(
    time: Res<Time>,
    mut wave_status: ResMut<WaveStatus>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    wave_status.timer.tick(time.delta());
    if wave_status.timer.finished() {
        next_state.set(AppState::LevelUp);
    }
}
