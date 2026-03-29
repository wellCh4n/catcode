use std::process::Command;

fn catcode() -> Command {
    Command::new(env!("CARGO_BIN_EXE_catcode"))
}

#[test]
fn test_file_and_dir_mutual_exclusion() {
    let output = catcode()
        .args(["-f", "main.rs", "-d", "."])
        .output()
        .expect("failed to execute catcode");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("either") || stderr.contains("both"));
}

#[test]
fn test_missing_file_and_dir() {
    let output = catcode()
        .args(&["--" as &str])
        .output()
        .expect("failed to execute catcode");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("provide") || stderr.contains("Error"));
}

#[test]
fn test_help_flag() {
    let output = catcode()
        .args(["--help"])
        .output()
        .expect("failed to execute catcode");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("-f") || stdout.contains("--file"));
    assert!(stdout.contains("-d") || stdout.contains("--dir"));
    assert!(stdout.contains("-i") || stdout.contains("--imports"));
    assert!(stdout.contains("-I") || stdout.contains("--inheritance"));
}
