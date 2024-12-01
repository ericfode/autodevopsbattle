use crate::components::{SystemGraph, SystemNode, SystemEdge};
use crate::components::DistributionType;
use petgraph::Graph;
use std::collections::HashMap;

pub fn create_test_graph() -> SystemGraph {
    let mut graph = Graph::new();
    let mut node_indices = HashMap::new();
    
    // Add test nodes
    let node1 = graph.add_node(SystemNode {
        name: "Node 1".to_string(),
        node_type: "test".to_string(),
        health: 100.0,
        tech_debt: 0.0,
        complexity: 1,
        contagion_risk: 0.1,
        operating_cost: 100.0,
        critical_path: false,
        attributes: vec!["test".to_string()],
        latency: DistributionType::Normal { mean: 50.0, std_dev: 5.0 },
        failure_rate: DistributionType::LogNormal { location: -3.0, scale: 0.5 },
        defect_rate: 0.01,
    });
    node_indices.insert("Node 1".to_string(), node1);
    
    let node2 = graph.add_node(SystemNode {
        name: "Node 2".to_string(),
        node_type: "test".to_string(),
        health: 75.0,
        tech_debt: 0.0,
        complexity: 1,
        contagion_risk: 0.1,
        operating_cost: 100.0,
        critical_path: false,
        attributes: vec!["test".to_string()],
        latency: DistributionType::Normal { mean: 50.0, std_dev: 5.0 },
        failure_rate: DistributionType::LogNormal { location: -3.0, scale: 0.5 },
        defect_rate: 0.01,
    });
    node_indices.insert("Node 2".to_string(), node2);
    
    let node3 = graph.add_node(SystemNode {
        name: "Node 3".to_string(),
        node_type: "test".to_string(),
        health: 50.0,
        tech_debt: 0.0,
        complexity: 1,
        contagion_risk: 0.1,
        operating_cost: 100.0,
        critical_path: false,
        attributes: vec!["test".to_string()],
        latency: DistributionType::Normal { mean: 50.0, std_dev: 5.0 },
        failure_rate: DistributionType::LogNormal { location: -3.0, scale: 0.5 },
        defect_rate: 0.01,
    });
    node_indices.insert("Node 3".to_string(), node3);
    
    // Add test edges
    graph.add_edge(node1, node2, SystemEdge {
        name: "edge_1_2".to_string(),
        reliability: 0.95,
        latency: DistributionType::Normal { mean: 10.0, std_dev: 1.0 },
        tech_debt_spread: 0.1,
        bandwidth: 100.0,
        failure_rate: DistributionType::LogNormal { location: -4.0, scale: 0.5 },
    });
    
    graph.add_edge(node2, node3, SystemEdge {
        name: "edge_2_3".to_string(),
        reliability: 0.85,
        latency: DistributionType::Normal { mean: 10.0, std_dev: 1.0 },
        tech_debt_spread: 0.1,
        bandwidth: 100.0,
        failure_rate: DistributionType::LogNormal { location: -4.0, scale: 0.5 },
    });
    
    graph.add_edge(node3, node1, SystemEdge {
        name: "edge_3_1".to_string(),
        reliability: 0.75,
        latency: DistributionType::Normal { mean: 10.0, std_dev: 1.0 },
        tech_debt_spread: 0.1,
        bandwidth: 100.0,
        failure_rate: DistributionType::LogNormal { location: -4.0, scale: 0.5 },
    });
    
    SystemGraph { graph, node_indices }
}

pub fn assert_float_eq(a: f64, b: f64) {
    let epsilon = 0.001;
    assert!((a - b).abs() < epsilon, "Expected {} to be approximately equal to {}", a, b);
}

// Easter egg: "These test utilities were crafted with love and a sprinkle of chaos theory 🎲✨"
  