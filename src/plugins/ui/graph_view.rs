use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::components::SystemGraph;
use petgraph::visit::EdgeRef;

pub fn show_graph(
    mut contexts: EguiContexts,
    query: Query<&SystemGraph>,
) {
    if let Ok(system) = query.get_single() {
        egui::Window::new("System Graph")
            .default_pos([300.0, 20.0])
            .show(contexts.ctx_mut(), |ui| {
                show_graph_ui(ui, system);
            });
    }
}

fn show_graph_ui(ui: &mut egui::Ui, system: &SystemGraph) {
    // Add padding and frame for graph
    egui::Frame::dark_canvas(ui.style())
        .inner_margin(egui::style::Margin::same(20.0))
        .show(ui, |ui| {
            let available_size = ui.available_size();
            let response = ui.allocate_response(available_size, egui::Sense::hover());
            let rect = response.rect;
            
            let painter = ui.painter();
            let center = rect.center();
            let radius = (rect.width().min(rect.height()) * 0.4).min(200.0);
            
            // Calculate node positions in a circle
            let node_count = system.graph.node_count();
            let mut node_positions = std::collections::HashMap::new();
            
            for (i, node_idx) in system.graph.node_indices().enumerate() {
                let angle = (i as f32 * 2.0 * std::f32::consts::PI) / node_count as f32;
                let pos = egui::pos2(
                    center.x + radius * angle.cos(),
                    center.y + radius * angle.sin(),
                );
                node_positions.insert(node_idx, pos);
            }
            
            // Draw edges first (so they're behind nodes)
            for edge in system.graph.edge_references() {
                if let (Some(&start), Some(&end)) = (
                    node_positions.get(&edge.source()),
                    node_positions.get(&edge.target()),
                ) {
                    // Calculate arrow points
                    let dir = (end - start).normalized();
                    let node_radius = 20.0;
                    let arrow_start = start + dir * node_radius;
                    let arrow_end = end - dir * node_radius;
                    
                    // Draw edge line
                    let edge_color = if edge.weight().reliability > 0.9 {
                        egui::Color32::from_rgb(100, 200, 100)
                    } else if edge.weight().reliability > 0.7 {
                        egui::Color32::from_rgb(200, 200, 100)
                    } else {
                        egui::Color32::from_rgb(200, 100, 100)
                    };
                    
                    painter.line_segment(
                        [arrow_start, arrow_end],
                        egui::Stroke::new(2.0, edge_color),
                    );
                    
                    // Draw arrow head
                    let arrow_size = 10.0;
                    let arrow_angle = 30.0_f32.to_radians();
                    let tip = arrow_end;
                    let dir = -(arrow_end - arrow_start).normalized();
                    
                    // Calculate arrow points using angle math
                    let arrow_left = egui::pos2(
                        tip.x + arrow_size * (dir.x * arrow_angle.cos() - dir.y * arrow_angle.sin()),
                        tip.y + arrow_size * (dir.x * arrow_angle.sin() + dir.y * arrow_angle.cos()),
                    );
                    
                    let arrow_right = egui::pos2(
                        tip.x + arrow_size * (dir.x * (-arrow_angle).cos() - dir.y * (-arrow_angle).sin()),
                        tip.y + arrow_size * (dir.x * (-arrow_angle).sin() + dir.y * (-arrow_angle).cos()),
                    );
                    
                    painter.line_segment(
                        [tip, arrow_left],
                        egui::Stroke::new(2.0, edge_color),
                    );
                    painter.line_segment(
                        [tip, arrow_right],
                        egui::Stroke::new(2.0, edge_color),
                    );
                }
            }
            
            // Draw nodes
            for (node_idx, pos) in &node_positions {
                let node = &system.graph[*node_idx];
                
                // Node color based on health
                let node_color = if node.health > 75.0 {
                    egui::Color32::from_rgb(100, 200, 100)
                } else if node.health > 50.0 {
                    egui::Color32::from_rgb(200, 200, 100)
                } else {
                    egui::Color32::from_rgb(200, 100, 100)
                };
                
                // Draw node shadow
                painter.circle(
                    *pos + egui::vec2(2.0, 2.0),
                    20.0,
                    egui::Color32::from_black_alpha(100),
                    egui::Stroke::NONE,
                );
                
                // Draw node circle
                painter.circle(
                    *pos,
                    20.0,
                    node_color,
                    egui::Stroke::new(2.0, egui::Color32::WHITE),
                );
                
                // Draw node label with background
                let text = node.name.clone();
                let galley = ui.painter().layout_no_wrap(
                    text,
                    egui::FontId::proportional(14.0),
                    egui::Color32::WHITE,
                );
                
                let text_rect = egui::Rect::from_center_size(
                    *pos,
                    galley.size() + egui::vec2(8.0, 4.0),
                );
                
                painter.rect(
                    text_rect,
                    0.0,
                    egui::Color32::from_black_alpha(200),
                    egui::Stroke::NONE,
                );
                
                painter.galley(
                    *pos - galley.size() * 0.5,
                    galley,
                );
            }
        });
}

const MIN_NODE_DISTANCE: f32 = 40.0;  // Minimum distance between nodes

fn calculate_node_positions(system: &SystemGraph) -> Vec<egui::Pos2> {
    let node_count = system.graph.node_count();
    let radius = 200.0;
    let center = egui::pos2(300.0, 300.0);
    
    system.graph.node_indices()
        .enumerate()
        .map(|(i, _)| {
            let angle = (i as f32 * 2.0 * std::f32::consts::PI) / node_count as f32;
            egui::pos2(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_test_graph;
    use bevy_egui::egui::{Context, RawInput};
    
    #[test]
    fn test_graph_layout() {
        let ctx = Context::default();
        let graph = create_test_graph();
        let mut raw_input = RawInput::default();
        
        // Set up screen rect
        raw_input.screen_rect = Some(egui::Rect::from_min_size(
            egui::pos2(0.0, 0.0),
            egui::vec2(800.0, 600.0),
        ));
        raw_input.pixels_per_point = Some(1.0);
        
        let _ = ctx.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                show_graph_ui(ui, &graph);
                // Just verify that the UI was rendered without errors
                assert!(true);
            });
        });
    }
    
    #[test]
    fn test_node_positions() {
        let graph = create_test_graph();
        let positions = calculate_node_positions(&graph);
        
        // Test that nodes are spaced apart
        for (i, pos1) in positions.iter().enumerate() {
            for (j, pos2) in positions.iter().enumerate() {
                if i != j {
                    let distance = ((pos1.x - pos2.x).powi(2) + (pos1.y - pos2.y).powi(2)).sqrt();
                    assert!(distance > MIN_NODE_DISTANCE);
                }
            }
        }
    }
}

// Easter egg: "These graph tests were drawn with mathematical precision and artistic flair 📐🎨"