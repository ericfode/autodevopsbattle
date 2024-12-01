use crate::components::{SystemGraph, SystemNode, SystemEdge};

pub fn create_test_graph() -> SystemGraph {
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

// Easter egg: "These test utilities were forged in the fires of Mount Debug 🌋" 