use bevy::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, RegisterInspectable, WorldInspectorPlugin};

use crate::{components::*, resources::*};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<AsciiSheet>()
                .register_inspectable::<CellComponent>()
                .add_plugin(InspectorPlugin::<Rule>::new());
        }
    }
}
