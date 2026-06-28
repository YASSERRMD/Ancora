use crate::crewai::{
    map_crewai_to_ancora, CrewAIAgent, CrewAIDefinition, CrewAITask, CrewMappingError,
};

#[test]
fn crewai_roles_map_to_a_crew() {
    let def = CrewAIDefinition {
        crew_name: "dev-crew".into(),
        agents: vec![
            CrewAIAgent {
                name: "dev".into(),
                role: "Developer".into(),
                goal: "Write code".into(),
                backstory: "Senior engineer".into(),
            },
            CrewAIAgent {
                name: "reviewer".into(),
                role: "Reviewer".into(),
                goal: "Review PRs".into(),
                backstory: "Staff engineer".into(),
            },
        ],
        tasks: vec![
            CrewAITask { description: "Implement feature X".into(), assigned_to: "dev".into() },
            CrewAITask { description: "Review PR".into(), assigned_to: "reviewer".into() },
        ],
    };
    let plan = map_crewai_to_ancora(def).unwrap();
    assert_eq!(plan.name, "dev-crew");
    assert_eq!(plan.members.len(), 2);
    assert_eq!(plan.task_assignments.len(), 2);
    assert_eq!(plan.task_assignments[0].1, "dev");
}

#[test]
fn crewai_empty_crew_returns_error() {
    let def = CrewAIDefinition {
        crew_name: "empty".into(),
        agents: vec![],
        tasks: vec![],
    };
    assert!(matches!(map_crewai_to_ancora(def), Err(CrewMappingError::EmptyCrew)));
}

#[test]
fn crewai_unknown_agent_in_task_returns_error() {
    let def = CrewAIDefinition {
        crew_name: "partial".into(),
        agents: vec![CrewAIAgent {
            name: "agent-a".into(),
            role: "Role A".into(),
            goal: "goal".into(),
            backstory: "backstory".into(),
        }],
        tasks: vec![CrewAITask {
            description: "task for ghost".into(),
            assigned_to: "ghost-agent".into(),
        }],
    };
    assert!(matches!(
        map_crewai_to_ancora(def),
        Err(CrewMappingError::UnknownAgent(_))
    ));
}

#[test]
fn crewai_member_objective_combines_goal_and_backstory() {
    let def = CrewAIDefinition {
        crew_name: "test".into(),
        agents: vec![CrewAIAgent {
            name: "m1".into(),
            role: "Tester".into(),
            goal: "Catch bugs".into(),
            backstory: "QA veteran".into(),
        }],
        tasks: vec![],
    };
    let plan = map_crewai_to_ancora(def).unwrap();
    assert!(plan.members[0].objective.contains("Catch bugs"));
    assert!(plan.members[0].objective.contains("QA veteran"));
}
