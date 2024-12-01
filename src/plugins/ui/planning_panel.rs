use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::resources::GameResources;
use crate::components::SystemGraph;
use crate::GameState;

pub fn show_planning_panel(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    resources: Res<GameResources>,
    query: Query<&SystemGraph>,
    state: Res<State<GameState>>,
) {
    egui::Window::new("Planning Phase 🎯")
        .default_pos([600.0, 20.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Sprint Planning");
            ui.add_space(8.0);
            
            // Sprint info
            ui.label(format!("Sprint {} Planning", resources.sprint));
            ui.add_space(16.0);
            
            // Available resources
            ui.group(|ui| {
                ui.label("Available Resources:");
                ui.label(format!("💰 Budget: ${:.2}", resources.money));
                ui.label(format!("⭐ Reputation: {:.1}%", resources.reputation));
            });
            ui.add_space(16.0);
            
            // System overview
            if let Ok(system) = query.get_single() {
                ui.group(|ui| {
                    ui.label("System Overview:");
                    ui.label(format!("🔧 Components: {}", system.graph.node_count()));
                    ui.label(format!("🔗 Dependencies: {}", system.graph.edge_count()));
                    
                    // Calculate average tech debt
                    let avg_tech_debt: f64 = system.graph.node_weights()
                        .map(|n| n.tech_debt)
                        .sum::<f64>() / system.graph.node_count() as f64;
                    ui.label(format!("🔥 Avg Tech Debt: {:.1}%", avg_tech_debt));
                });
            }
            ui.add_space(16.0);
            
            // Action buttons with state awareness
            ui.horizontal(|ui| {
                let current_state = state.get();
                
                if ui.button("⏸️ Pause Simulation").clicked() {
                    if *current_state != GameState::Paused {
                        info!("User paused simulation");
                        next_state.set(GameState::Paused);
                    } else {
                        info!("Simulation already paused");
                    }
                }
                
                if ui.button("▶️ Start Simulation").clicked() {
                    if *current_state != GameState::Running {
                        info!("User started simulation");
                        next_state.set(GameState::Running);
                    } else {
                        info!("Simulation already running");
                    }
                }
            });
            
            // Current state indicator
            ui.add_space(8.0);
            ui.label(format!("Current State: {:?}", state.get()));
            
            // Planning tips
            ui.add_space(16.0);
            ui.group(|ui| {
                ui.label("💡 Tips:");
                ui.label("• Review system health before proceeding");
                ui.label("• Consider tech debt impact");
                ui.label("• Balance short-term vs long-term");
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::test_utils::setup_test_app;

    #[test]
    fn test_resource_display() {
        let mut app = setup_test_app();
        // Modify resources for testing
        let mut resources = app.world.resource_mut::<GameResources>();
        resources.money = 5000.0;
        resources.reputation = 75.0;
        resources.sprint = 2;
        
        // Run the system
        app.update();
        
        // Verify resources are correct
        let resources = app.world.resource::<GameResources>();
        assert_eq!(resources.money, 5000.0);
        assert_eq!(resources.reputation, 75.0);
        assert_eq!(resources.sprint, 2);
    }

    #[test]
    fn test_state_transitions() {
        let mut app = setup_test_app();
        app.add_state::<GameState>();
        // Test initial state
        let state = app.world.resource::<State<GameState>>();
        assert_eq!(*state.get(), GameState::Loading);
        
        // Run the system once
        app.update();
    }
}

// Easter egg: "This planning panel was crafted during a perfectly timed coffee break ☕" 