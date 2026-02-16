use std::process::Command;

fn run_cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_poker_eval_rs_cli"))
        .args(args)
        .output()
        .expect("failed to run poker_eval_rs_cli")
}

#[test]
fn cli_rejects_board_for_stud7() {
    let output = run_cli(&[
        "equity",
        "AsKsQh",
        "AdKdQc",
        "--game",
        "stud7",
        "--board",
        "2c",
        "--monte-carlo",
        "--iterations",
        "32",
    ]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr).to_lowercase();
    assert!(stderr.contains("invalid board configuration"));
}

#[test]
fn cli_rejects_invalid_exhaustive_street_for_holdem() {
    let output = run_cli(&[
        "equity", "AsKs", "QhQd", "--game", "holdem", "--board", "2c7d",
    ]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr).to_lowercase();
    assert!(stderr.contains("invalid board configuration"));
}
