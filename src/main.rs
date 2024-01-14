use bevy::{prelude::*, time::common_conditions::on_timer, window::PresentMode};
use components::{
    CollideCircle, GameSystemSet, Health, Lifetime, MainCamera, PhysicalObj, UniformVelocity,
};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use show_debug::ShowDebugPlugin;
use show_fps::ShowFpsPlugin;
use spatial_hashmap::{SpatialHashmap, SquareQuery};
use std::time::Duration;

mod camera;
mod components;
mod enemy;
mod player;
mod show_debug;
mod show_fps;
pub mod spatial_hashmap;

const SHM_GRID_SIZE: f32 = 5.0;

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
    shm: SpatialHashmap,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            68.0 / 225.0,
            36.0 / 255.0,
            52.0 / 255.0,
        )))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevyruman".into(),
                resolution: (1280f32, 720f32).into(),
                present_mode: PresentMode::AutoNoVsync, //fps見るため,vsync off
                ..Default::default()
            }),
            ..Default::default()
        }))
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
            shm: SpatialHashmap::new(SHM_GRID_SIZE),
        })
        .add_plugins((ShowDebugPlugin, ShowFpsPlugin))
        .add_plugins((PlayerPlugin, EnemyPlugin))
        .add_systems(Startup, setup_system)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(
            Update,
            (physical_obj_pre_proc_system, update_lifetime_system)
                .in_set(GameSystemSet::PreProcess),
        )
        .add_systems(
            Update,
            uniform_linear_motion_system.in_set(GameSystemSet::Update),
        )
        .add_systems(
            Update,
            (
                //collision_detection_system,
                collision_detection_shm_system,
            )
                .in_set(GameSystemSet::UpdatePhysics),
        )
        .add_systems(
            Update,
            physical_obj_do_verlet_system.in_set(GameSystemSet::PostPhysics),
        )
        .add_systems(
            Update,
            update_health_system.in_set(GameSystemSet::PostUpdate),
        )
        .add_systems(
            PostUpdate,
            camera::update_camera_system.run_if(on_timer(Duration::from_secs_f32(1. / 60.))),
        )
        .run();
}

fn setup_system(mut commands: Commands) {
    // camera
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn physical_obj_pre_proc_system(mut query: Query<(&Transform, &mut PhysicalObj)>) {
    for (transform, mut obj) in query.iter_mut() {
        obj.move_vec = Vec2::ZERO;
        obj.old_move_vec = Vec2::ZERO;
        obj.force = Vec2::ZERO;
        obj.velocity = transform.translation.xy() - obj.old_pos;
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
            //p1->m_hit_mask.set(p2->m_colli_attr);
            // p2,apply impulse
            obj1.old_move_vec -= n * (impulse_j * inv_mass1);
            //p2->m_hit_mask.set(p1->m_colli_attr);
        }
    }
}

fn collision_detection_shm_system(
    #[allow(unused_mut)] mut query: Query<(Entity, &Transform, &CollideCircle, &mut PhysicalObj)>,
    shm: ResMut<SHM>,
) {
    unsafe {
        for (e0, tf0, colli0, mut obj0) in query.iter_unsafe() {
            let square_query = SquareQuery::new(tf0.translation.xy(), colli0.radius);

            for (e1, _position) in shm.shm.query(square_query) {
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
                        //p1->m_hit_mask.set(p2->m_colli_attr);
                        // p2,apply impulse
                        obj1.old_move_vec -= n * (impulse_j * inv_mass1);
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
    mut shm: ResMut<SHM>,
    mut query: Query<(Entity, &mut PhysicalObj, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    if dt <= 0. {
        return;
    };
    let inv_prev_dt = 1. / physics_resource.prev_dt;
    let damping = 0.4;
    let decel = f32::powf(damping, dt);
    for (entity, mut obj, mut transform) in query.iter_mut() {
        let pos = transform.translation.xy() + obj.move_vec;
        let mut tmp = obj.old_pos + obj.move_vec;
        tmp = tmp + obj.old_move_vec; //change velocity

        // do verlet
        let vel = (pos - tmp) * inv_prev_dt;
        let inv_mass_dt = obj.inv_mass * dt;
        let vel = vel + obj.force * inv_mass_dt;
        let vel = vel * decel; //damping

        let tmp = pos + vel * dt;

        // shm更新
        shm.shm
            .update(entity, transform.translation.truncate(), tmp);
        // set_position
        let translation = &mut transform.translation;
        *translation = tmp.extend(translation.z);
        obj.old_pos = pos;
        // set_velocity
        obj.velocity = vel;
    }
    physics_resource.prev_dt = dt;
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

// 生存時間
fn update_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

// 体力更新
fn update_health_system(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, &ref health) in query.iter() {
        if health.0 <= 0. {
            commands.entity(entity).despawn();
        }
    }
}
