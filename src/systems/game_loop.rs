use bevy::prelude::*;
use crate::components::SystemGraph;
use crate::resources::GameResources;
use crate::components::ArchitectureType;
use petgraph::visit::EdgeRef;
use std::time::Duration;

pub fn tick_system(
    time: Res<Time>,
    mut query: Query<&mut SystemGraph>,
    mut resources: ResMut<GameResources>,
) {
    // Get the time delta
    let dt = time.delta_seconds_f64();
    println!("Time delta: {}", dt);  // Debug output
    
    // Update each system graph
    for mut system in query.iter_mut() {
        // Update node health based on tech debt
        for node_idx in system.graph.node_indices() {
            let node = &mut system.graph[node_idx];
            
            // Health decreases faster with higher tech debt
            let health_decay = node.tech_debt * 0.1 * dt;  // 10% of tech debt per second
            let old_health = node.health;
            node.health = (node.health - health_decay).max(0.0);
            println!("Node health: {} -> {} (decay: {})", old_health, node.health, health_decay);  // Debug output
            
            // Operating costs increase with tech debt
            let cost_multiplier = 1.0 + (node.tech_debt / 100.0);  // Up to 2x cost at 100% tech debt
            let old_money = resources.money;
            resources.money -= node.operating_cost * cost_multiplier * dt;
            println!("Money: {} -> {}", old_money, resources.money);  // Debug output
            
            // Critical path nodes affect reputation
            if node.critical_path && node.health < 50.0 {
                resources.reputation -= (50.0 - node.health) * 0.1 * dt;  // Lose reputation faster for unhealthy critical nodes
            }
        }
        
        // Simulate tech debt spread through edges
        let mut tech_debt_changes = Vec::new();
        for edge in system.graph.edge_references() {
            let source_idx = edge.source();
            let target_idx = edge.target();
            let source = &system.graph[source_idx];
            
            // Tech debt spreads based on the edge's tech_debt_spread factor
            let spread_amount = source.tech_debt * edge.weight().tech_debt_spread * dt;
            tech_debt_changes.push((target_idx, spread_amount));
            println!("Tech debt spread: {} -> {}", source.name, spread_amount);  // Debug output
        }
        
        // Apply tech debt changes
        for (node_idx, spread_amount) in tech_debt_changes {
            let node = &mut system.graph[node_idx];
            let old_tech_debt = node.tech_debt;
            node.tech_debt = (node.tech_debt + spread_amount).min(100.0);
            println!("Node {} tech debt: {} -> {}", node.name, old_tech_debt, node.tech_debt);  // Debug output
        }
    }
    
    // Clamp resources to valid ranges
    resources.money = resources.money.max(0.0);
    resources.reputation = resources.reputation.clamp(0.0, 100.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_test_graph, assert_float_eq};
    
    #[test]
    fn test_game_loop_health() {
        let graph = create_test_graph();
        let mut found_nodes = [false; 3];
        
        // Test initial health
        for node_idx in graph.graph.node_indices() {
            let node = &graph.graph[node_idx];
            match node.name.as_str() {
                "Node 1" => {
                    assert_float_eq(node.health, 100.0_f64);
                    found_nodes[0] = true;
                },
                "Node 2" => {
                    assert_float_eq(node.health, 75.0_f64);
                    found_nodes[1] = true;
                },
                "Node 3" => {
                    assert_float_eq(node.health, 50.0_f64);
                    found_nodes[2] = true;
                },
                _ => panic!("Unexpected node name: {}", node.name),
            }
            assert_float_eq(node.tech_debt, 0.0_f64);
        }
        
        // Ensure we found all nodes
        assert!(found_nodes.iter().all(|&found| found), "Not all expected nodes were found");
    }
    
    #[test]
    fn test_tick_system() {
        // Set up test environment
        let mut app = App::new();
        
        // Initialize time with non-zero delta
        let mut time = Time::default();
        time.update();  // First update to initialize
        std::thread::sleep(Duration::from_millis(16));  // Simulate 16ms frame time
        time.update();  // Second update to get non-zero delta
        println!("Initial time delta: {}", time.delta_seconds_f64());  // Debug output
        
        app.insert_resource(time)
           .insert_resource(GameResources {
               money: 1000.0,
               reputation: 100.0,
               sprint: 1,
               current_architecture: ArchitectureType::Monolith,
           })
           .add_systems(Update, tick_system);
        
        // Add test graph
        let mut graph = create_test_graph();
        
        // Add some initial tech debt to test spread
        if let Some(node_idx) = graph.node_indices.get("Node 1") {
            graph.graph[*node_idx].tech_debt = 50.0;
            println!("Initial tech debt for Node 1: {}", graph.graph[*node_idx].tech_debt);  // Debug output
        }
        
        app.world.spawn(graph);
        
        // Run one tick
        app.update();
        
        // Get updated graph
        let updated_graph = app.world.query::<&SystemGraph>().single(&app.world);
        
        // Verify tech debt has spread and health has decreased
        if let Some(node_idx) = updated_graph.node_indices.get("Node 1") {
            let node = &updated_graph.graph[*node_idx];
            println!("Final health for Node 1: {}", node.health);  // Debug output
            assert!(node.health < 100.0, "Health should decrease due to tech debt");
        }
        
        // Verify resources have been affected
        let resources = app.world.resource::<GameResources>();
        println!("Final money: {}", resources.money);  // Debug output
        assert!(resources.money < 1000.0, "Operating costs should reduce money");
    }
}

// Easter egg: "This game loop was crafted with love and a sprinkle of chaos theory 🎮✨"