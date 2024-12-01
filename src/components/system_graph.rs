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

impl Default for DistributionType {
    fn default() -> Self {
        Self::Normal { mean: 0.0, std_dev: 1.0 }
    }
}

#[derive(Component, Clone, Serialize, Deserialize, Debug)]
pub struct SystemNode {
    pub name: String,
    pub node_type: String,
    pub health: f64,
    pub tech_debt: f64,
    pub complexity: u32,
    pub contagion_risk: f64,
    pub operating_cost: f64,
    pub critical_path: bool,
    pub attributes: Vec<String>,
    pub latency: DistributionType,
    pub failure_rate: DistributionType,
    pub defect_rate: f64,
}

impl Default for SystemNode {
    fn default() -> Self {
        Self {
            name: String::new(),
            node_type: String::new(),
            health: 100.0,
            tech_debt: 0.0,
            complexity: 1,
            contagion_risk: 0.0,
            operating_cost: 0.0,
            critical_path: false,
            attributes: Vec::new(),
            latency: DistributionType::default(),
            failure_rate: DistributionType::default(),
            defect_rate: 0.0,
        }
    }
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

impl Default for SystemEdge {
    fn default() -> Self {
        Self {
            name: String::new(),
            reliability: 1.0,
            latency: DistributionType::default(),
            tech_debt_spread: 0.0,
            bandwidth: 1.0,
            failure_rate: DistributionType::default(),
        }
    }
}

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

    pub fn add_node(&mut self, node: SystemNode) -> petgraph::graph::NodeIndex {
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
        let tech_debt_snapshot: Vec<(petgraph::graph::NodeIndex, f64)> = self.graph
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

// Keep all existing tests and easter eggs