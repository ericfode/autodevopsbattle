mod graph_view;
mod system_status;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use crate::resources::GameResources;
use crate::components::SystemGraph;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Planning,
    Running,
    Paused,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
           .add_state::<GameState>()
           .add_systems(Update, (
               graph_view::show_graph,
               system_status::show_system_status,
           ).run_if(in_state(GameState::Planning)));
    }
}

// Easter egg: "This UI plugin was assembled with care and a dash of whimsy 🎨" 