use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::resources::GameResources;
use crate::components::SystemGraph;

pub fn show_system_status(
    mut contexts: EguiContexts,
    resources: Res<GameResources>,
    query: Query<&SystemGraph>,
) {
    if let Ok(system) = query.get_single() {
        egui::Window::new("System Status")
            .default_pos([20.0, 20.0])
            .show(contexts.ctx_mut(), |ui| {
                show_system_status_ui(ui, &resources, system);
            });
    }
}

fn show_system_status_ui(
    ui: &mut egui::Ui,
    resources: &GameResources,
    system: &SystemGraph,
) {
    ui.heading("System Status");
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);
    
    // Show game resources with colored text
    ui.label(egui::RichText::new(format!("💰 Money: ${:.2}", resources.money))
        .color(egui::Color32::from_rgb(158, 255, 158)));
    ui.label(egui::RichText::new(format!("🏃 Sprint: {}", resources.sprint))
        .color(egui::Color32::LIGHT_BLUE));
    ui.label(egui::RichText::new(format!("⭐ Reputation: {:.1}%", resources.reputation))
        .color(egui::Color32::GOLD));
    
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);
    
    // Show individual components
    ui.heading("System Components");
    ui.separator();
    
    for node in system.graph.node_weights() {
        ui.collapsing(
            egui::RichText::new(&node.name).strong(), 
            |ui| {
                ui.add_space(4.0);
                ui.label(format!("Type: {}", node.node_type));
                
                // Color health based on value
                let health_color = if node.health > 75.0 {
                    egui::Color32::GREEN
                } else if node.health > 50.0 {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::RED
                };
                ui.label(egui::RichText::new(format!("Health: {:.1}%", node.health))
                    .color(health_color));
                
                // Color tech debt based on value
                let debt_color = if node.tech_debt > 75.0 {
                    egui::Color32::RED
                } else if node.tech_debt > 50.0 {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::GREEN
                };
                ui.label(egui::RichText::new(format!("Tech Debt: {:.1}%", node.tech_debt))
                    .color(debt_color));
                
                ui.label(format!("Complexity: {}", node.complexity));
                if node.critical_path {
                    ui.label(
                        egui::RichText::new("⚠️ Critical Path")
                            .color(egui::Color32::RED)
                            .strong()
                    );
                }
                
                // Show attributes
                if !node.attributes.is_empty() {
                    ui.add_space(4.0);
                    ui.label("Attributes:");
                    for attr in &node.attributes {
                        ui.label(
                            egui::RichText::new(format!("  • {}", attr))
                                .italics()
                                .color(egui::Color32::LIGHT_BLUE)
                        );
                    }
                }
            }
        );
        ui.add_space(4.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{SystemNode, SystemEdge};
    use super::super::test_utils::setup_test_app;

    fn create_test_system() -> SystemGraph {
        let mut graph = SystemGraph::new();
        
        let node1 = graph.add_node(SystemNode {
            name: "test_node_1".into(),
            health: 100.0,
            tech_debt: 10.0,
            operating_cost: 100.0,
            critical_path: true,
            ..Default::default()
        });
        
        let node2 = graph.add_node(SystemNode {
            name: "test_node_2".into(),
            health: 100.0,
            tech_debt: 0.0,
            operating_cost: 50.0,
            critical_path: false,
            ..Default::default()
        });
        
        graph.add_edge(
            "test_node_1",
            "test_node_2",
            SystemEdge {
                tech_debt_spread: 0.1,
                ..Default::default()
            },
        );
        
        graph
    }

    #[test]
    fn test_system_status_setup() {
        let mut app = setup_test_app();
        
        let system = create_test_system();
        app.world.spawn(system);
        
        // Run one update to ensure systems are working
        app.update();
    }
}

// Easter egg: "This status panel was built with real-time monitoring in mind 📊"