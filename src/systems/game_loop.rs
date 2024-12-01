use bevy::prelude::*;
use crate::resources::GameResources;
use crate::components::SystemGraph;
use crate::GameState;
use petgraph::visit::EdgeRef;

pub fn tick_system(
    time: Res<Time>,
    mut resources: ResMut<GameResources>,
    mut query: Query<&mut SystemGraph>,
    state: Res<State<GameState>>,
) {
    // Only run simulation in Running state
    if *state.get() != GameState::Running {
        return;
    }

    let delta = time.delta_seconds_f64();
    trace!("Simulation tick: delta = {:.4}s", delta);
    
    // Update system state
    if let Ok(mut system) = query.get_single_mut() {
        // Track changes for logging
        let initial_money = resources.money;
        let initial_reputation = resources.reputation;
        let mut total_health_decay = 0.0;
        let mut total_tech_debt_spread = 0.0;
        
        // Update node health based on tech debt
        for node_idx in system.graph.node_indices() {
            let node = &mut system.graph[node_idx];
            
            // Health decreases faster with higher tech debt
            let health_decay = node.tech_debt * 0.1 * delta;  // 10% of tech debt per second
            total_health_decay += health_decay;
            
            let old_health = node.health;
            node.health = (node.health - health_decay).max(0.0);
            trace!("Node {} health: {:.2} -> {:.2} (decay: {:.2})", 
                node.name, old_health, node.health, health_decay);

            let cost_multiplier = 1.0 + (node.tech_debt / 100.0);  // Up to 2x cost at 100% tech debt
            resources.money -= node.operating_cost * cost_multiplier * delta;
            
            // Critical path nodes affect reputation
            if node.critical_path && node.health < 50.0 {
                let reputation_loss = (50.0 - node.health) * 0.1 * delta;
                resources.reputation -= reputation_loss;
                debug!("Critical node {} below 50% health, reputation loss: {:.2}", 
                    node.name, reputation_loss);
            }
        }

        // Process tech debt spread
        let mut tech_debt_changes = Vec::new();
        for edge in system.graph.edge_references() {
            let source = &system.graph[edge.source()];
            let spread_amount = source.tech_debt * edge.weight().tech_debt_spread * delta;
            total_tech_debt_spread += spread_amount;
            tech_debt_changes.push((edge.target(), spread_amount));
        }
        
        // Apply tech debt changes
        for (target_idx, spread_amount) in tech_debt_changes {
            let target = &mut system.graph[target_idx];
            let old_tech_debt = target.tech_debt;
            target.tech_debt = (target.tech_debt + spread_amount).min(100.0);
            trace!("Tech debt spread to {}: {:.2} -> {:.2}", 
                target.name, old_tech_debt, target.tech_debt);
        }
        
        // Log summary of changes
        info!("Simulation update - Money: ${:.2} -> ${:.2}, Reputation: {:.1}% -> {:.1}%", 
            initial_money, resources.money, initial_reputation, resources.reputation);
        debug!("System health decay: {:.2}, Tech debt spread: {:.2}", 
            total_health_decay, total_tech_debt_spread);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;
    use bevy::ecs::system::SystemState;
    use crate::components::{SystemNode, SystemEdge};
    use petgraph::Graph;
    
    // Helper function to create a test system
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
    fn test_simulation_paused() {
        let mut app = App::new();
        app.add_state::<GameState>()
            .insert_resource(Time::default())
            .insert_resource(GameResources::default());
            
        let system = create_test_system();
        app.world.spawn(system);
        
        // Set state to Paused
        app.world.insert_resource(State::new(GameState::Paused));
        
        // Run the system
        let mut system_state: SystemState<(
            Res<Time>,
            ResMut<GameResources>,
            Query<&mut SystemGraph>,
            Res<State<GameState>>,
        )> = SystemState::new(&mut app.world);
        
        let (time, mut resources, mut query, state) = system_state.get_mut(&mut app.world);
        
        // Store initial values
        let initial_money = resources.money;
        let initial_reputation = resources.reputation;
        
        // Run tick system
        tick_system(time, resources, query, state);
        
        // Get updated values
        let (_, resources, _, _) = system_state.get_mut(&mut app.world);
        
        // Verify nothing changed while paused
        assert_eq!(resources.money, initial_money);
        assert_eq!(resources.reputation, initial_reputation);
        
        system_state.apply(&mut app.world);
    }
    
    #[test]
    fn test_simulation_running() {
        let mut app = App::new();
        let mut time = Time::default();
        time.update();  // This sets last_update to now
        time.set_relative_speed(1.0);  // Ensure normal time speed
        std::thread::sleep(std::time::Duration::from_millis(16));  // Simulate one frame
        time.update();  // This creates a non-zero delta
        
        app.add_state::<GameState>()
            .insert_resource(time)
            .insert_resource(GameResources::default());
            
        let system = create_test_system();
        app.world.spawn(system);
        
        // Set state to Running
        app.world.insert_resource(State::new(GameState::Running));
        
        // Run the system
        let mut system_state: SystemState<(
            Res<Time>,
            ResMut<GameResources>,
            Query<&mut SystemGraph>,
            Res<State<GameState>>,
        )> = SystemState::new(&mut app.world);
        
        let (time, mut resources, mut query, state) = system_state.get_mut(&mut app.world);
        
        // Store initial values
        let initial_money = resources.money;
        
        // Run tick system
        tick_system(time, resources, query, state);
        
        // Get updated values
        let (_, resources, query, _) = system_state.get_mut(&mut app.world);
        
        // Verify simulation had an effect
        assert!(resources.money < initial_money, "Operating costs should reduce money");
        
        // Verify tech debt spread
        if let Ok(system) = query.get_single() {
            let node2_idx = system.node_indices.get("test_node_2").unwrap();
            let node2 = &system.graph[*node2_idx];
            assert!(node2.tech_debt > 0.0, "Tech debt should spread to second node");
        }
        
        system_state.apply(&mut app.world);
    }
}

// Easter egg: "This game loop was crafted with love and a sprinkle of chaos theory 🎮✨"