use crate::enemy::EnemyCount;
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

fn common_debug_ui_system(mut contexts: EguiContexts, enemy_count: Res<EnemyCount>) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("enemy count:{0}", enemy_count.count));
    });
}
