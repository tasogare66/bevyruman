use crate::{enemy::EnemyCount, GameConfig};
use bevy::prelude::*;
use bevy::render::view::screenshot::ScreenshotManager;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct DwGuiPlugin;

impl Plugin for DwGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
            // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
            .add_systems(Update, common_debug_ui_system);
    }
}

fn common_debug_ui_system(
    mut contexts: EguiContexts,
    mut commands: Commands,
    enemy_count: Res<EnemyCount>,
    mut game_config_query: Query<&mut GameConfig>,
    //screenshot
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
) {
    let mut game_config = game_config_query.get_single_mut().unwrap();
    egui::Window::new("debug").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("enemy count:{0}", enemy_count.count));
        if ui.button("screenshot").clicked() {
            //FIXME:screenshot取れない
            let path = format!("./ram/screenshot-{}.png", *counter);
            *counter += 1;
            screenshot_manager
                .save_screenshot_to_disk(main_window.single(), path)
                .unwrap();
        }
        ui.horizontal(|ui| {
            ui.checkbox(&mut game_config.dbg_show_collision, "show_collision");
            ui.checkbox(&mut game_config.dbg_least_time, "least time");
        });
        if ui.button("Reset").clicked() {
            *game_config = GameConfig::default();
        }
        ui.horizontal(|ui| {
            if ui.button("Load").clicked() {
                commands.insert_resource(crate::LoadConfigRequest);
            }
            if ui.button("Save").clicked() {
                commands.insert_resource(crate::SaveConfigRequest);
            }
        });
    });
}
