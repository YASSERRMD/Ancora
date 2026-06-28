/// Eval run CLI - command-line interface for running evaluations.

use crate::executor::{EvalCase, Executor, RunConfig, RunId, exact_match};
use crate::rollout::RolloutRunner;
use crate::aggregate::compute_aggregate;
use crate::breakdown::{compute_breakdown, sort_by_pass_rate_asc};
use crate::report::EvalReport;

/// CLI command variants.
#[derive(Debug, Clone)]
pub enum CliCommand {
    Run {
        suite_name: String,
        n_rollouts: usize,
        seed: u64,
        output_format: OutputFormat,
    },
    Compare {
        run_id_a: String,
        run_id_b: String,
    },
    List,
}

/// Output format for eval reports.
#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Json,
    Html,
    Text,
}

/// Parse a simple CLI argument list into a CliCommand.
///
/// Supported forms:
///   run <suite> [--rollouts N] [--seed S] [--format json|html|text]
///   compare <run_a> <run_b>
///   list
pub fn parse_args(args: &[&str]) -> Result<CliCommand, String> {
    if args.is_empty() {
        return Err("no command provided".to_string());
    }
    match args[0] {
        "run" => {
            if args.len() < 2 {
                return Err("run requires a suite name".to_string());
            }
            let suite_name = args[1].to_string();
            let mut n_rollouts = 1usize;
            let mut seed = 42u64;
            let mut output_format = OutputFormat::Text;
            let mut i = 2;
            while i < args.len() {
                match args[i] {
                    "--rollouts" => {
                        i += 1;
                        n_rollouts = args.get(i).ok_or("missing value for --rollouts")?
                            .parse::<usize>()
                            .map_err(|e| e.to_string())?;
                    }
                    "--seed" => {
                        i += 1;
                        seed = args.get(i).ok_or("missing value for --seed")?
                            .parse::<u64>()
                            .map_err(|e| e.to_string())?;
                    }
                    "--format" => {
                        i += 1;
                        output_format = match args.get(i).copied() {
                            Some("json") => OutputFormat::Json,
                            Some("html") => OutputFormat::Html,
                            Some("text") | None => OutputFormat::Text,
                            Some(f) => return Err(format!("unknown format: {}", f)),
                        };
                    }
                    other => return Err(format!("unknown flag: {}", other)),
                }
                i += 1;
            }
            Ok(CliCommand::Run { suite_name, n_rollouts, seed, output_format })
        }
        "compare" => {
            if args.len() < 3 {
                return Err("compare requires two run IDs".to_string());
            }
            Ok(CliCommand::Compare {
                run_id_a: args[1].to_string(),
                run_id_b: args[2].to_string(),
            })
        }
        "list" => Ok(CliCommand::List),
        other => Err(format!("unknown command: {}", other)),
    }
}

/// Run a fixture eval suite from the CLI and return the report.
///
/// `cases` and `infer` are injected so the CLI stays testable without real inference.
pub fn run_suite_from_cli<F>(
    command: &CliCommand,
    cases: &[EvalCase],
    infer: &F,
    timestamp: u64,
) -> Result<String, String>
where
    F: Fn(&str, u64) -> (String, u64, u64),
{
    match command {
        CliCommand::Run { suite_name, n_rollouts, seed, output_format } => {
            let run_id = RunId(format!("{}-{}", suite_name, seed));
            let config = RunConfig {
                run_id: run_id.clone(),
                scorer: exact_match,
                seed: *seed,
            };
            let executor = Executor::new(config);
            let runner = RolloutRunner::new(*n_rollouts);
            let rollouts = runner.rollout_suite(&executor, cases, infer);
            let metrics = compute_aggregate(&rollouts);
            let mut breakdowns = compute_breakdown(&rollouts);
            sort_by_pass_rate_asc(&mut breakdowns);

            let report = EvalReport::new(
                run_id,
                suite_name.clone(),
                timestamp,
                metrics,
                breakdowns,
            );

            match output_format {
                OutputFormat::Json => Ok(report.to_json()),
                OutputFormat::Html => Ok(report.to_html()),
                OutputFormat::Text => Ok(format!(
                    "Eval run complete: suite={} pass_rate={:.3}",
                    suite_name,
                    report.metrics.pass_rate,
                )),
            }
        }
        CliCommand::Compare { run_id_a, run_id_b } => {
            Ok(format!("compare {} vs {}", run_id_a, run_id_b))
        }
        CliCommand::List => Ok("no stored runs".to_string()),
    }
}
