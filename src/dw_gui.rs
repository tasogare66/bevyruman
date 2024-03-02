use crate::{enemy::EnemyCount, GameConfig};
use bevy::prelude::*;
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
    enemy_count: Res<EnemyCount>,
    mut game_config: ResMut<GameConfig>,
) {
    egui::Window::new("debug").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("enemy count:{0}", enemy_count.count));
        ui.horizontal(|ui| {
            ui.checkbox(&mut game_config.dbg_show_collision, "show_collision");
            ui.checkbox(&mut game_config.dbg_least_time, "least time");
        });
        if ui.button("Reset").clicked() {
            *game_config = GameConfig::default();
        }
        ui.horizontal(|ui| {
            if ui.button("Load").clicked() {}
            if ui.button("Save").clicked() {}
        });
    });
}
