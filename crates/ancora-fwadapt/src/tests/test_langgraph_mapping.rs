use crate::langgraph::{
    map_langgraph_to_stages, GraphMappingError, LangGraphDefinition, LangGraphEdge, LangGraphNode,
};

#[test]
fn langgraph_mapping_executes_linear() {
    let g = LangGraphDefinition {
        nodes: vec![
            LangGraphNode {
                id: "start".into(),
                label: "Start".into(),
            },
            LangGraphNode {
                id: "end".into(),
                label: "End".into(),
            },
        ],
        edges: vec![LangGraphEdge {
            from: "start".into(),
            to: "end".into(),
        }],
        entry: "start".into(),
    };
    let stages = map_langgraph_to_stages(&g).unwrap();
    assert_eq!(stages.len(), 2);
    assert_eq!(stages[0].order, 0);
    assert_eq!(stages[1].order, 1);
}

#[test]
fn langgraph_bad_entry_returns_error() {
    let g = LangGraphDefinition {
        nodes: vec![LangGraphNode {
            id: "a".into(),
            label: "A".into(),
        }],
        edges: vec![],
        entry: "missing".into(),
    };
    assert!(matches!(
        map_langgraph_to_stages(&g),
        Err(GraphMappingError::EntryNotFound(_))
    ));
}

#[test]
fn langgraph_cycle_returns_error() {
    let g = LangGraphDefinition {
        nodes: vec![
            LangGraphNode {
                id: "x".into(),
                label: "X".into(),
            },
            LangGraphNode {
                id: "y".into(),
                label: "Y".into(),
            },
        ],
        edges: vec![
            LangGraphEdge {
                from: "x".into(),
                to: "y".into(),
            },
            LangGraphEdge {
                from: "y".into(),
                to: "x".into(),
            },
        ],
        entry: "x".into(),
    };
    assert!(matches!(
        map_langgraph_to_stages(&g),
        Err(GraphMappingError::CycleDetected)
    ));
}

#[test]
fn langgraph_three_node_chain_stages_ordered() {
    let g = LangGraphDefinition {
        nodes: vec![
            LangGraphNode {
                id: "a".into(),
                label: "Alpha".into(),
            },
            LangGraphNode {
                id: "b".into(),
                label: "Beta".into(),
            },
            LangGraphNode {
                id: "c".into(),
                label: "Gamma".into(),
            },
        ],
        edges: vec![
            LangGraphEdge {
                from: "a".into(),
                to: "b".into(),
            },
            LangGraphEdge {
                from: "b".into(),
                to: "c".into(),
            },
        ],
        entry: "a".into(),
    };
    let stages = map_langgraph_to_stages(&g).unwrap();
    assert_eq!(stages[0].name, "Alpha");
    assert_eq!(stages[2].name, "Gamma");
}
