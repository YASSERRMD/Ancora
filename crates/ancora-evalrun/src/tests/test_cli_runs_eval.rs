use crate::cli::{parse_args, run_suite_from_cli, CliCommand};
use crate::executor::EvalCase;

fn fixture_cases() -> Vec<EvalCase> {
    vec![
        EvalCase {
            id: "cli-c1".into(),
            input: "foo".into(),
            expected: "bar".into(),
        },
        EvalCase {
            id: "cli-c2".into(),
            input: "baz".into(),
            expected: "qux".into(),
        },
    ]
}

fn perfect_infer(input: &str, _seed: u64) -> (String, u64, u64) {
    let out = match input {
        "foo" => "bar",
        "baz" => "qux",
        _ => "",
    };
    (out.into(), 10, 5)
}

#[test]
fn cli_parse_run_command() {
    let args = vec!["run", "my-suite", "--rollouts", "3", "--seed", "7"];
    let cmd = parse_args(&args).expect("should parse");
    match cmd {
        CliCommand::Run {
            suite_name,
            n_rollouts,
            seed,
            ..
        } => {
            assert_eq!(suite_name, "my-suite");
            assert_eq!(n_rollouts, 3);
            assert_eq!(seed, 7);
        }
        _ => panic!("expected Run command"),
    }
}

#[test]
fn cli_parse_compare_command() {
    let args = vec!["compare", "run-001", "run-002"];
    let cmd = parse_args(&args).expect("should parse");
    match cmd {
        CliCommand::Compare { run_id_a, run_id_b } => {
            assert_eq!(run_id_a, "run-001");
            assert_eq!(run_id_b, "run-002");
        }
        _ => panic!("expected Compare command"),
    }
}

#[test]
fn cli_parse_list_command() {
    let args = vec!["list"];
    let cmd = parse_args(&args).expect("should parse");
    assert!(matches!(cmd, CliCommand::List));
}

#[test]
fn cli_parse_unknown_command_errors() {
    let args = vec!["bogus"];
    assert!(parse_args(&args).is_err());
}

#[test]
fn cli_runs_eval_text_output() {
    let args = vec!["run", "cli-suite", "--rollouts", "2"];
    let cmd = parse_args(&args).expect("should parse");
    let cases = fixture_cases();
    let output = run_suite_from_cli(&cmd, &cases, &perfect_infer, 1000).expect("should succeed");
    assert!(
        output.contains("cli-suite"),
        "output should mention suite name"
    );
    assert!(
        output.contains("pass_rate"),
        "output should mention pass_rate"
    );
}

#[test]
fn cli_runs_eval_json_output() {
    let args = vec!["run", "cli-suite", "--rollouts", "2", "--format", "json"];
    let cmd = parse_args(&args).expect("should parse");
    let cases = fixture_cases();
    let output = run_suite_from_cli(&cmd, &cases, &perfect_infer, 1000).expect("should succeed");
    assert!(output.starts_with('{'), "JSON output should start with {{");
    assert!(
        output.contains("pass_rate"),
        "JSON should contain pass_rate"
    );
}

#[test]
fn cli_runs_eval_html_output() {
    let args = vec!["run", "cli-suite", "--rollouts", "1", "--format", "html"];
    let cmd = parse_args(&args).expect("should parse");
    let cases = fixture_cases();
    let output = run_suite_from_cli(&cmd, &cases, &perfect_infer, 1000).expect("should succeed");
    assert!(output.contains("<!DOCTYPE html>"), "output should be HTML");
}
