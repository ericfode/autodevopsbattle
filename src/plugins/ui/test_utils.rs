use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;
use bevy_egui::{egui, EguiContext, EguiSettings};
use crate::resources::GameResources;

pub struct TestPlugins;

impl PluginGroup for TestPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(bevy::log::LogPlugin::default())
            .add(bevy::core::TaskPoolPlugin::default())
            .add(bevy::core::TypeRegistrationPlugin::default())
            .add(bevy::core::FrameCountPlugin::default())
            .add(bevy::time::TimePlugin::default())
    }
}

#[derive(Resource)]
struct MockEguiContext {
    ctx: egui::Context,
}

impl Default for MockEguiContext {
    fn default() -> Self {
        Self {
            ctx: egui::Context::default(),
        }
    }
}

impl bevy_egui::EguiContexts for MockEguiContext {
    fn ctx_mut(&mut self) -> &mut egui::Context {
        &mut self.ctx
    }

    fn ctx(&self) -> &egui::Context {
        &self.ctx
    }
}

pub fn setup_test_app() -> App {
    let mut app = App::new();
    
    app.add_plugins(TestPlugins)
       .insert_resource(GameResources::default())
       .insert_resource(EguiSettings::default())
       .insert_non_send_resource(MockEguiContext::default());

    app
} 