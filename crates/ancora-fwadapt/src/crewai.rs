//! Adapters for mapping CrewAI role definitions into Ancora crew models.
//!
//! CrewAI organises agents as a named crew where each member has a role,
//! goal, and backstory. This module maps those concepts to Ancora's agent
//! primitive without requiring the Python CrewAI runtime.

#[derive(Debug, Clone, PartialEq)]
pub struct CrewAIAgent {
    pub name: String,
    pub role: String,
    pub goal: String,
    pub backstory: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrewAITask {
    pub description: String,
    pub assigned_to: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrewAIDefinition {
    pub crew_name: String,
    pub agents: Vec<CrewAIAgent>,
    pub tasks: Vec<CrewAITask>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AncoraCrewMember {
    pub id: String,
    pub role: String,
    pub objective: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AncoraCrewPlan {
    pub name: String,
    pub members: Vec<AncoraCrewMember>,
    pub task_assignments: Vec<(String, String)>, // (task_description, member_id)
}

#[derive(Debug, Clone, PartialEq)]
pub enum CrewMappingError {
    UnknownAgent(String),
    EmptyCrew,
}

impl std::fmt::Display for CrewMappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownAgent(a) => write!(f, "unknown agent in task assignment: {}", a),
            Self::EmptyCrew => write!(f, "crew has no agents"),
        }
    }
}

/// Convert a CrewAI definition into an Ancora crew plan.
pub fn map_crewai_to_ancora(crew: CrewAIDefinition) -> Result<AncoraCrewPlan, CrewMappingError> {
    if crew.agents.is_empty() {
        return Err(CrewMappingError::EmptyCrew);
    }

    let members: Vec<AncoraCrewMember> = crew
        .agents
        .iter()
        .map(|a| AncoraCrewMember {
            id: a.name.clone(),
            role: a.role.clone(),
            objective: format!("{} - {}", a.goal, a.backstory),
        })
        .collect();

    let member_ids: std::collections::HashSet<&str> =
        members.iter().map(|m| m.id.as_str()).collect();

    let mut task_assignments = Vec::new();
    for task in &crew.tasks {
        if !member_ids.contains(task.assigned_to.as_str()) {
            return Err(CrewMappingError::UnknownAgent(task.assigned_to.clone()));
        }
        task_assignments.push((task.description.clone(), task.assigned_to.clone()));
    }

    Ok(AncoraCrewPlan {
        name: crew.crew_name,
        members,
        task_assignments,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_crewai_definition() {
        let def = CrewAIDefinition {
            crew_name: "research-crew".into(),
            agents: vec![CrewAIAgent {
                name: "analyst".into(),
                role: "Researcher".into(),
                goal: "Find insights".into(),
                backstory: "Expert in data".into(),
            }],
            tasks: vec![CrewAITask {
                description: "Analyse Q1 data".into(),
                assigned_to: "analyst".into(),
            }],
        };
        let plan = map_crewai_to_ancora(def).unwrap();
        assert_eq!(plan.name, "research-crew");
        assert_eq!(plan.members.len(), 1);
        assert_eq!(plan.task_assignments[0].1, "analyst");
    }
}
