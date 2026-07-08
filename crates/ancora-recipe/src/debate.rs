use crate::format::{Recipe, RecipeStep, StepAction};
use crate::params::ParamSet;

/// Build a multi-agent debate recipe.
pub fn build(params: &ParamSet) -> Recipe {
    let rounds: usize = params
        .get("rounds")
        .and_then(|v| v.parse().ok())
        .unwrap_or(2);
    let agents: usize = params
        .get("agents")
        .and_then(|v| v.parse().ok())
        .unwrap_or(2);

    let mut r = Recipe::new(
        "multi-agent-debate",
        "Multi-Agent Debate",
        format!(
            "Run a {}-round debate among {} agents and produce a consensus verdict.",
            rounds, agents
        ),
    );

    r.add_step(RecipeStep::new(
        "setup",
        StepAction::Generate,
        format!("Assign roles and opening positions to {} agents", agents),
    ));

    for round in 1..=rounds {
        r.add_step(RecipeStep::new(
            format!("round-{}", round),
            StepAction::Generate,
            format!(
                "Run debate round {}: each agent responds to opponents",
                round
            ),
        ));
    }

    r.add_step(RecipeStep::new(
        "verdict",
        StepAction::Summarize,
        "Synthesize debate into a consensus verdict or dissent summary",
    ));
    r
}

/// A debate agent's stance on a proposition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stance {
    For,
    Against,
    Neutral,
}

/// A single argument made during a debate.
#[derive(Debug, Clone)]
pub struct Argument {
    pub agent_id: usize,
    pub round: usize,
    pub stance: Stance,
    pub text: String,
}

impl Argument {
    pub fn new(agent_id: usize, round: usize, stance: Stance, text: impl Into<String>) -> Self {
        Self {
            agent_id,
            round,
            stance,
            text: text.into(),
        }
    }
}

/// Count arguments by stance.
pub fn count_by_stance(args: &[Argument]) -> (usize, usize, usize) {
    let for_count = args.iter().filter(|a| a.stance == Stance::For).count();
    let against = args.iter().filter(|a| a.stance == Stance::Against).count();
    let neutral = args.iter().filter(|a| a.stance == Stance::Neutral).count();
    (for_count, against, neutral)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::ParamSet;

    #[test]
    fn build_recipe_step_count() {
        let mut params = ParamSet::default();
        params.set("rounds", "2");
        params.set("agents", "3");
        let r = build(&params);
        // setup + 2 rounds + verdict = 4
        assert_eq!(r.step_count(), 4);
        assert!(r.validate().is_ok());
    }

    #[test]
    fn count_stances() {
        let args = vec![
            Argument::new(0, 1, Stance::For, "arg1"),
            Argument::new(1, 1, Stance::Against, "arg2"),
            Argument::new(0, 2, Stance::For, "arg3"),
        ];
        let (f, a, n) = count_by_stance(&args);
        assert_eq!(f, 2);
        assert_eq!(a, 1);
        assert_eq!(n, 0);
    }
}
