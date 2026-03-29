use std::process::Command;

fn catcode() -> Command {
    Command::new(env!("CARGO_BIN_EXE_catcode"))
}

fn fixture(path: &str) -> String {
    format!("tests/rust/{}", path)
}

// ============ Feature: List Methods (default) ============
#[test]
fn test_rust_list_methods() {
    let output = catcode()
        .args(["-f", &fixture("lib.rs")])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("create_user"));
    assert!(stdout.contains("get_user"));
    assert!(stdout.contains("delete_user"));
}

// ============ Feature: Method Detail ============
#[test]
fn test_rust_method_detail() {
    let output = catcode()
        .args(["-f", &fixture("lib.rs"), "-m", "create_user"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("create_user"));
    assert!(stdout.contains("L")); // line number
}

// ============ Feature: List Classes/Impls ============
#[test]
fn test_rust_list_classes() {
    let output = catcode()
        .args(["-f", &fixture("lib.rs"), "-c"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
    assert!(stdout.contains("User"));
}

// ============ Feature: Class Skeleton ============
#[test]
fn test_rust_class_skeleton() {
    let output = catcode()
        .args(["-f", &fixture("lib.rs"), "-c", "UserService"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
    assert!(stdout.contains("create_user"));
}

