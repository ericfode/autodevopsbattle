use super::system_graph::{SystemGraph, SystemNode, SystemEdge, DistributionType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchitectureType {
    Monolith,
    Microservices,
    EventDriven,
}

impl ArchitectureType {
    pub fn next(&self) -> Self {
        match self {
            Self::Monolith => Self::Microservices,
            Self::Microservices => Self::EventDriven,
            Self::EventDriven => Self::Monolith,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Monolith => "Monolith",
            Self::Microservices => "Microservices",
            Self::EventDriven => "Event-Driven",
        }
    }
}

pub fn create_architecture(arch_type: ArchitectureType) -> SystemGraph {
    match arch_type {
        ArchitectureType::Monolith => create_monolith(),
        ArchitectureType::Microservices => create_microservices(),
        ArchitectureType::EventDriven => create_event_driven(),
    }
}

fn create_monolith() -> SystemGraph {
    let mut graph = SystemGraph::new();
    
    let core = SystemNode {
        name: "core_service".into(),
        node_type: "monolith".into(),
        health: 100.0,
        tech_debt: 30.0,
        complexity: 15,
        contagion_risk: 0.5,
        operating_cost: 500.0,
        critical_path: true,
        attributes: vec!["monolithic".into(), "legacy".into()],
        latency: DistributionType::Normal { mean: 200.0, std_dev: 50.0 },
        failure_rate: DistributionType::LogNormal { location: -3.0, scale: 0.5 },
        defect_rate: 0.2,
    };
    
    let db = SystemNode {
        name: "database".into(),
        node_type: "storage".into(),
        health: 100.0,
        tech_debt: 20.0,
        complexity: 5,
        contagion_risk: 0.3,
        operating_cost: 300.0,
        critical_path: true,
        attributes: vec!["data_critical".into()],
        latency: DistributionType::Normal { mean: 50.0, std_dev: 10.0 },
        failure_rate: DistributionType::LogNormal { location: -4.0, scale: 0.3 },
        defect_rate: 0.1,
    };
    
    let cache = SystemNode {
        name: "cache".into(),
        node_type: "cache".into(),
        health: 100.0,
        tech_debt: 10.0,
        complexity: 3,
        contagion_risk: 0.2,
        operating_cost: 100.0,
        critical_path: false,
        attributes: vec!["performance".into()],
        latency: DistributionType::Normal { mean: 5.0, std_dev: 1.0 },
        failure_rate: DistributionType::LogNormal { location: -2.0, scale: 0.8 },
        defect_rate: 0.05,
    };
    
    let core_idx = graph.add_node(core);
    let db_idx = graph.add_node(db);
    let cache_idx = graph.add_node(cache);
    
    let core_to_db = SystemEdge {
        name: "db_connection".into(),
        reliability: 0.999,
        latency: DistributionType::Normal { mean: 10.0, std_dev: 2.0 },
        tech_debt_spread: 0.3,
        bandwidth: 1000.0,
        failure_rate: DistributionType::LogNormal { location: -5.0, scale: 0.2 },
    };
    
    let core_to_cache = SystemEdge {
        name: "cache_connection".into(),
        reliability: 0.99,
        latency: DistributionType::Normal { mean: 2.0, std_dev: 0.5 },
        tech_debt_spread: 0.1,
        bandwidth: 5000.0,
        failure_rate: DistributionType::LogNormal { location: -3.0, scale: 0.5 },
    };
    
    graph.graph.add_edge(core_idx, db_idx, core_to_db);
    graph.graph.add_edge(core_idx, cache_idx, core_to_cache);
    
    graph
}

fn create_microservices() -> SystemGraph {
    let mut graph = SystemGraph::new();
    
    // Create nodes
    let nodes = vec![
        ("gateway", SystemNode {
            name: "api_gateway".into(),
            node_type: "gateway".into(),
            health: 100.0,
            tech_debt: 15.0,
            complexity: 8,
            contagion_risk: 0.4,
            operating_cost: 200.0,
            critical_path: true,
            attributes: vec!["entry_point".into()],
            latency: DistributionType::Normal { mean: 50.0, std_dev: 10.0 },
            failure_rate: DistributionType::LogNormal { location: -4.0, scale: 0.3 },
            defect_rate: 0.1,
        }),
        ("auth", SystemNode {
            name: "auth_service".into(),
            node_type: "service".into(),
            health: 100.0,
            tech_debt: 20.0,
            complexity: 6,
            contagion_risk: 0.3,
            operating_cost: 150.0,
            critical_path: true,
            attributes: vec!["security".into()],
            latency: DistributionType::Normal { mean: 100.0, std_dev: 20.0 },
            failure_rate: DistributionType::LogNormal { location: -4.5, scale: 0.2 },
            defect_rate: 0.15,
        }),
        ("users", SystemNode {
            name: "user_service".into(),
            node_type: "service".into(),
            health: 100.0,
            tech_debt: 25.0,
            complexity: 7,
            contagion_risk: 0.3,
            operating_cost: 180.0,
            critical_path: true,
            attributes: vec!["core_service".into()],
            latency: DistributionType::Normal { mean: 80.0, std_dev: 15.0 },
            failure_rate: DistributionType::LogNormal { location: -4.0, scale: 0.3 },
            defect_rate: 0.12,
        }),
    ];
    
    // Add nodes and store indices
    let mut indices = std::collections::HashMap::new();
    for (key, node) in nodes {
        let idx = graph.add_node(node);
        indices.insert(key, idx);
    }
    
    // Create and add edges
    let edges = vec![
        (indices["gateway"], indices["auth"], SystemEdge {
            name: "gateway_to_auth".into(),
            reliability: 0.999,
            latency: DistributionType::Normal { mean: 20.0, std_dev: 5.0 },
            tech_debt_spread: 0.2,
            bandwidth: 1000.0,
            failure_rate: DistributionType::LogNormal { location: -5.0, scale: 0.2 },
        }),
        (indices["gateway"], indices["users"], SystemEdge {
            name: "gateway_to_users".into(),
            reliability: 0.999,
            latency: DistributionType::Normal { mean: 20.0, std_dev: 5.0 },
            tech_debt_spread: 0.2,
            bandwidth: 1000.0,
            failure_rate: DistributionType::LogNormal { location: -5.0, scale: 0.2 },
        }),
        (indices["auth"], indices["users"], SystemEdge {
            name: "auth_to_users".into(),
            reliability: 0.999,
            latency: DistributionType::Normal { mean: 30.0, std_dev: 8.0 },
            tech_debt_spread: 0.3,
            bandwidth: 500.0,
            failure_rate: DistributionType::LogNormal { location: -4.5, scale: 0.3 },
        }),
    ];
    
    for (from, to, edge) in edges {
        graph.graph.add_edge(from, to, edge);
    }
    
    graph
}

fn create_event_driven() -> SystemGraph {
    let mut graph = SystemGraph::new();
    
    let event_bus = SystemNode {
        name: "event_bus".into(),
        node_type: "messaging".into(),
        health: 100.0,
        tech_debt: 15.0,
        complexity: 10,
        contagion_risk: 0.6,
        operating_cost: 400.0,
        critical_path: true,
        attributes: vec!["backbone".into(), "distributed".into()],
        latency: DistributionType::Normal { mean: 30.0, std_dev: 10.0 },
        failure_rate: DistributionType::LogNormal { location: -5.0, scale: 0.2 },
        defect_rate: 0.1,
    };
    
    let producer = SystemNode {
        name: "producer_service".into(),
        node_type: "service".into(),
        health: 100.0,
        tech_debt: 20.0,
        complexity: 6,
        contagion_risk: 0.3,
        operating_cost: 200.0,
        critical_path: true,
        attributes: vec!["event_source".into()],
        latency: DistributionType::Normal { mean: 50.0, std_dev: 15.0 },
        failure_rate: DistributionType::LogNormal { location: -4.0, scale: 0.3 },
        defect_rate: 0.15,
    };
    
    let consumer = SystemNode {
        name: "consumer_service".into(),
        node_type: "service".into(),
        health: 100.0,
        tech_debt: 25.0,
        complexity: 7,
        contagion_risk: 0.4,
        operating_cost: 250.0,
        critical_path: true,
        attributes: vec!["event_sink".into()],
        latency: DistributionType::Normal { mean: 70.0, std_dev: 20.0 },
        failure_rate: DistributionType::LogNormal { location: -3.5, scale: 0.4 },
        defect_rate: 0.2,
    };
    
    let bus_idx = graph.add_node(event_bus);
    let producer_idx = graph.add_node(producer);
    let consumer_idx = graph.add_node(consumer);
    
    // Bidirectional connections for event bus
    let to_bus = SystemEdge {
        name: "to_bus".into(),
        reliability: 0.999,
        latency: DistributionType::Normal { mean: 15.0, std_dev: 5.0 },
        tech_debt_spread: 0.4,
        bandwidth: 2000.0,
        failure_rate: DistributionType::LogNormal { location: -4.5, scale: 0.3 },
    };
    
    let from_bus = SystemEdge {
        name: "from_bus".into(),
        reliability: 0.999,
        latency: DistributionType::Normal { mean: 15.0, std_dev: 5.0 },
        tech_debt_spread: 0.4,
        bandwidth: 2000.0,
        failure_rate: DistributionType::LogNormal { location: -4.5, scale: 0.3 },
    };
    
    graph.graph.add_edge(producer_idx, bus_idx, to_bus.clone());
    graph.graph.add_edge(bus_idx, consumer_idx, from_bus.clone());
    
    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_architecture_creation() {
        // Test Monolith
        let monolith = create_monolith();
        assert_eq!(monolith.graph.node_count(), 3);
        assert_eq!(monolith.graph.edge_count(), 2);
        assert!(monolith.graph.node_weights().any(|n| n.node_type == "monolith"));
        
        // Test Microservices
        let microservices = create_microservices();
        assert_eq!(microservices.graph.node_count(), 3);
        assert_eq!(microservices.graph.edge_count(), 3);
        assert!(microservices.graph.node_weights().any(|n| n.name == "api_gateway"));
        
        // Test Event-Driven
        let event_driven = create_event_driven();
        assert_eq!(event_driven.graph.node_count(), 3);
        assert_eq!(event_driven.graph.edge_count(), 2);
        assert!(event_driven.graph.node_weights().any(|n| n.name == "event_bus"));
    }
    
    #[test]
    fn test_architecture_type_cycle() {
        let mut arch = ArchitectureType::Monolith;
        arch = arch.next();
        assert_eq!(arch, ArchitectureType::Microservices);
        arch = arch.next();
        assert_eq!(arch, ArchitectureType::EventDriven);
        arch = arch.next();
        assert_eq!(arch, ArchitectureType::Monolith);
    }
} 