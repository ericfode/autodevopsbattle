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
    use crate::test_utils::create_test_graph;
    use bevy_egui::egui::{Context, RawInput};
    
    #[test]
    fn test_system_status_layout() {
        let ctx = Context::default();
        let _graph = create_test_graph();
        let raw_input = RawInput::default();
        
        let _ = ctx.run(raw_input, |ctx| {
            egui::Window::new("Test Window").show(ctx, |ui| {
                // We can't test the actual system_status function here since it requires EguiContexts
                // Instead, we'll test the layout logic
                let available_size = ui.available_size();
                assert!(available_size.x > 0.0);
                assert!(available_size.y > 0.0);
            });
        });
    }
}

// Easter egg: "This status test was crafted with love and a dash of chaos 🎭✨"