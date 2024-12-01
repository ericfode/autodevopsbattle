use bevy::prelude::*;
use crate::components::{SystemGraph, SystemNode, SystemEdge, DistributionType, ArchitectureType, create_architecture};

#[derive(Resource)]
pub struct GameResources {
    pub money: f64,
    pub sprint: u32,
    pub reputation: f64,
    pub current_architecture: ArchitectureType,
}

impl Default for GameResources {
    fn default() -> Self {
        Self {
            money: 10000.0,
            sprint: 1,
            reputation: 50.0,
            current_architecture: ArchitectureType::Monolith,
        }
    }
}

pub fn create_initial_system() -> SystemGraph {
    create_architecture(ArchitectureType::Monolith)
}

// Easter egg: The mythical perfect system
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_initial_system_creation() {
        let graph = create_initial_system();
        assert_eq!(graph.graph.node_count(), 3);
        assert_eq!(graph.graph.edge_count(), 2);
        
        // Verify the coffee machine isn't secretly part of the initial system
        assert!(graph.graph.node_weights()
            .all(|node| node.name != "coffee_machine"));
    }
} 