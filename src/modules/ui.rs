//! The User Interface

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
//use bevy_editor_pls::prelude::*; // Wait til this is in crates.io
use bevy_inspector_egui::WorldInspectorPlugin;

pub mod plugins;


pub struct Ui;

impl Ui {
    pub fn run() {
        App::new()
            .add_plugin(plugins::WindowDescriptorPlugin) // Must be first
            
            .add_plugins(DefaultPlugins) // Default Bevy plugins
            .add_plugin(EguiPlugin) // Default Egui plugins
            //.add_plugin(EditorPlugin) // Wait til this is in crates.io
            .add_plugin(WorldInspectorPlugin::new()) // bevy_inspector_egui plugin

            // Chui's plugins
            .add_plugin(plugins::GameStatePlugin)
            .add_plugin(plugins::UiStatePlugin)
            .add_plugin(plugins::CameraControllerPlugin)
            .add_plugin(plugins::AssetsPlugin)
            .add_plugin(plugins::MainUiPlugin)
            .run();
    }
}
