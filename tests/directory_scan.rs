use std::process::Command;

fn catcode() -> Command {
    Command::new(env!("CARGO_BIN_EXE_catcode"))
}

#[test]
fn test_directory_scan() {
    let output = catcode()
        .args(["-d", "tests"])
        .output()
        .expect("failed to execute catcode");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should find python files
    assert!(stdout.contains("PYTHON"));
    // Should find typescript files
    assert!(stdout.contains("TYPESCRIPT"));
}

#[test]
fn test_directory_scan_with_max_files() {
    let output = catcode()
        .args(["-d", "tests/python", "-x", "1"])
        .output()
        .expect("failed to execute catcode");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("more"));
}

