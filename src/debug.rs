use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};

use crate::{components::*, resources::*};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // only add plugins if the program is compiled in debug mode
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<AsciiSheet>()
                .register_inspectable::<CellComponent>()
                .add_plugin(InspectorPlugin::<GameOptions>::new())
                .add_system(history_panel_system);
        }
    }
}

fn history_panel_system(mut egui_ctx: ResMut<EguiContext>, mut history: ResMut<History>) {
    egui::SidePanel::right("History Panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("History");
            if ui.button("Clear history").clicked() {
                history.0.clear();
            }
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            for s in history.0.iter() {
                ui.label(s);
            }
        });
    });
}
