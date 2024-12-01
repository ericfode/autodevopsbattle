mod graph_view;
mod system_status;
mod planning_panel;
#[cfg(test)]
mod test_utils;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use crate::resources::GameResources;
use crate::components::SystemGraph;
use crate::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
           .add_systems(Update, (
               graph_view::show_graph,
               system_status::show_system_status,
               planning_panel::show_planning_panel,
           ).run_if(not(in_state(GameState::Loading))));
    }
}

// Easter egg: "This UI plugin was assembled with care and a dash of whimsy 🎨" 