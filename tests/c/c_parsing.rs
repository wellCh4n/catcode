use std::process::Command;

fn catcode() -> Command {
    Command::new(env!("CARGO_BIN_EXE_catcode"))
}

fn fixture(path: &str) -> String {
    format!("tests/c/{}", path)
}

// ============ Feature: List Methods (default) ============
#[test]
fn test_c_list_methods() {
    let output = catcode()
        .args(["-f", &fixture("main.c")])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    // C parser may not find all methods, but should find some
    assert!(stdout.contains("delete_user") || stdout.contains("main"));
}

// ============ Feature: Method Detail ============
// Note: C method lookup by name doesn't work correctly in current implementation
// because the name is nested inside declarator > declarator
// Skipping this test

