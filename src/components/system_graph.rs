use bevy::prelude::*;

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use rand_distr::{Distribution, Normal, LogNormal};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Distribution wrapper that can be serialized
#[derive(Component, Clone, Serialize, Deserialize, Debug)]
pub enum DistributionType {
    Normal { mean: f64, std_dev: f64 },
    LogNormal { location: f64, scale: f64 },
    // Add more as needed
}

impl DistributionType {
    pub fn sample(&self) -> f64 {
        use rand::thread_rng;
        match self {
            Self::Normal { mean, std_dev } => {
                Normal::new(*mean, *std_dev)
                    .map(|d| d.sample(&mut thread_rng()))
                    .unwrap_or(*mean)
            }
            Self::LogNormal { location, scale } => {
                LogNormal::new(*location, *scale)
                    .map(|d| d.sample(&mut thread_rng()))
                    .unwrap_or(*location)
            }
        }
    }
}

#[derive(Component, Clone, Serialize, Deserialize, Debug)]
pub struct SystemNode {
    pub name: String,
    pub node_type: String,
    pub health: f64,  // 0-100%
    pub tech_debt: f64,
    pub complexity: u32,
    pub contagion_risk: f64,
    pub operating_cost: f64,
    pub critical_path: bool,
    pub attributes: Vec<String>,
    
    // Performance characteristics
    pub latency: DistributionType,
    pub failure_rate: DistributionType,
    pub defect_rate: f64,
}

#[derive(Component, Clone, Serialize, Deserialize, Debug)]
pub struct SystemEdge {
    pub name: String,
    pub reliability: f64,
    pub latency: DistributionType,
    pub tech_debt_spread: f64,
    pub bandwidth: f64,
    pub failure_rate: DistributionType,
}

// The main system graph component
#[derive(Component)]
pub struct SystemGraph {
    pub graph: DiGraph<SystemNode, SystemEdge>,
    pub node_indices: HashMap<String, NodeIndex>,
}

impl SystemGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_indices: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: SystemNode) -> NodeIndex {
        let name = node.name.clone();
        let idx = self.graph.add_node(node);
        self.node_indices.insert(name, idx);
        idx
    }

    pub fn add_edge(&mut self, 
        from: &str, 
        to: &str, 
        edge: SystemEdge
    ) -> Option<()> {
        let from_idx = self.node_indices.get(from)?;
        let to_idx = self.node_indices.get(to)?;
        self.graph.add_edge(*from_idx, *to_idx, edge);
        Some(())
    }

    // Calculate total system complexity
    pub fn total_complexity(&self) -> u32 {
        self.graph.node_weights()
            .map(|node| node.complexity)
            .sum()
    }

    // Calculate average tech debt
    pub fn average_tech_debt(&self) -> f64 {
        let nodes = self.graph.node_weights().count();
        if nodes == 0 { return 0.0; }
        
        self.graph.node_weights()
            .map(|node| node.tech_debt)
            .sum::<f64>() / nodes as f64
    }

    // Simulate tech debt spread for one tick
    pub fn simulate_tech_debt_spread(&mut self) {
        // Clone the current tech debt values
        let tech_debt_snapshot: Vec<(NodeIndex, f64)> = self.graph
            .node_indices()
            .map(|idx| (idx, self.graph[idx].tech_debt))
            .collect();

        // Calculate new tech debt based on connections
        for (node_idx, current_debt) in tech_debt_snapshot {
            let node = &self.graph[node_idx];
            let contagion_risk = node.contagion_risk;
            
            // Get all incoming edges and their source nodes
            let incoming_debt: f64 = self.graph
                .edges_directed(node_idx, petgraph::Direction::Incoming)
                .map(|edge| {
                    let source_node = &self.graph[edge.source()];
                    let edge_data = edge.weight();
                    
                    source_node.tech_debt * 
                    edge_data.tech_debt_spread * 
                    contagion_risk * 
                    (1.0 + source_node.complexity as f64 / 10.0)
                })
                .sum();

            // Update the node's tech debt
            let new_debt = (current_debt + incoming_debt).min(100.0);
            self.graph[node_idx].tech_debt = new_debt;
        }
    }

    // Generate defects based on current state
    pub fn generate_defects(&mut self) -> Vec<(String, u32)> {
        self.graph
            .node_weights_mut()
            .filter_map(|node| {
                let base_rate = node.defect_rate;
                let tech_debt_factor = node.tech_debt / 100.0;
                let complexity_multiplier = 1.0 + (node.complexity as f64 / 10.0);
                
                let defect_count = (base_rate * 
                    (1.0 + tech_debt_factor).powi(2) * 
                    complexity_multiplier) as u32;
                
                if defect_count > 0 {
                    Some((node.name.clone(), defect_count))
                } else {
                    None
                }
            })
            .collect()
    }
}

// Easter egg: Hidden in the comments
/*
   🎮 Game Design Secret #42:
   Every time you call simulate_tech_debt_spread(),
   there's a 0.1% chance that it actually improves the system.
   Because sometimes, chaos works in our favor.
   
   Also, if you're reading this, you've found an easter egg!
   The coffee machine microservice is actually running on this code.
*/ 

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use test_case::test_case;

    // Strategy for generating valid SystemNode names
    fn node_name_strategy() -> impl Strategy<Value = String> {
        "[A-Za-z][A-Za-z0-9_]{0,20}".prop_filter(
            "Node names must be valid identifiers",
            |s| !s.contains(' ')
        )
    }

    // Strategy for generating valid SystemNodes
    fn system_node_strategy() -> impl Strategy<Value = SystemNode> {
        (
            node_name_strategy(),
            "[A-Za-z]{3,10}",  // node_type
            0.0..100.0f64,     // health
            0.0..100.0f64,     // tech_debt
            0u32..100u32,      // complexity
            0.0..1.0f64,       // contagion_risk
            1.0..1000.0f64,    // operating_cost
            any::<bool>(),     // critical_path
            prop::collection::vec("[A-Za-z]{3,10}", 0..5),  // attributes
        ).prop_map(|(name, node_type, health, tech_debt, complexity, 
                    contagion_risk, operating_cost, critical_path, attributes)| {
            SystemNode {
                name,
                node_type,
                health,
                tech_debt,
                complexity,
                contagion_risk,
                operating_cost,
                critical_path,
                attributes,
                latency: DistributionType::Normal { mean: 100.0, std_dev: 10.0 },
                failure_rate: DistributionType::LogNormal { location: -3.0, scale: 0.5 },
                defect_rate: 0.1,
            }
        })
    }

    proptest! {
        // Test that tech debt never exceeds 100%
        #[test]
        fn test_tech_debt_bounds(
            nodes in prop::collection::vec(system_node_strategy(), 1..10)
        ) {
            let mut graph = SystemGraph::new();
            
            // Add all nodes
            for node in nodes {
                graph.add_node(node);
            }
            
            // Simulate tech debt spread
            graph.simulate_tech_debt_spread();
            
            // Verify bounds
            for node_idx in graph.graph.node_indices() {
                let node = &graph.graph[node_idx];
                assert!(node.tech_debt >= 0.0 && node.tech_debt <= 100.0);
            }
        }

        // Test that complexity calculations are consistent
        #[test]
        fn test_complexity_consistency(
            nodes in prop::collection::vec(system_node_strategy(), 1..10)
        ) {
            let mut graph = SystemGraph::new();
            let total_input_complexity: u32 = nodes.iter()
                .map(|n| n.complexity)
                .sum();
            
            // Add all nodes
            for node in nodes {
                graph.add_node(node);
            }
            
            assert_eq!(graph.total_complexity(), total_input_complexity);
        }
    }

    // Regular unit tests using test-case
    #[test_case(0.0, 0.0 ; "zero tech debt")]
    #[test_case(50.0, 50.0 ; "medium tech debt")]
    #[test_case(100.0, 100.0 ; "maximum tech debt")]
    fn test_average_tech_debt_single_node(input: f64, expected: f64) {
        let mut graph = SystemGraph::new();
        let node = SystemNode {
            name: "test".into(),
            node_type: "service".into(),
            health: 100.0,
            tech_debt: input,
            complexity: 1,
            contagion_risk: 0.0,
            operating_cost: 100.0,
            critical_path: false,
            attributes: vec![],
            latency: DistributionType::Normal { mean: 100.0, std_dev: 10.0 },
            failure_rate: DistributionType::LogNormal { location: -3.0, scale: 0.5 },
            defect_rate: 0.1,
        };
        
        graph.add_node(node);
        assert_eq!(graph.average_tech_debt(), expected);
    }
}

// Easter egg: Hidden test case
#[cfg(test)]
#[test]
fn test_coffee_machine_microservice() {
    let mut graph = SystemGraph::new();
    let coffee = SystemNode {
        name: "coffee_machine".into(),
        node_type: "critical_infrastructure".into(),
        health: 100.0,
        tech_debt: 0.0,  // It's perfect, don't touch it
        complexity: 9000, // It's over 9000!
        contagion_risk: 0.0,
        operating_cost: 42.0,
        critical_path: true, // Of course it is
        attributes: vec!["do_not_touch".into(), "actually_works".into()],
        latency: DistributionType::Normal { mean: 180.0, std_dev: 10.0 }, // 3 minutes to brew
        failure_rate: DistributionType::LogNormal { location: -99.9, scale: 0.1 }, // Never fails
        defect_rate: 0.0,  // It's not a bug, it's a feature
    };
    
    graph.add_node(coffee);
    assert!(graph.graph.node_weights().any(|n| n.name == "coffee_machine"));
}