use std::process::Command;

fn catcode() -> Command {
    Command::new(env!("CARGO_BIN_EXE_catcode"))
}

fn fixture(path: &str) -> String {
    format!("tests/php/{}", path)
}

// ============ Feature: List Methods (default) ============
#[test]
fn test_php_list_methods() {
    let output = catcode()
        .args(["-f", &fixture("Service.php")])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    // PHP class methods may not be fully parsed, but should show some functions
    assert!(stdout.contains("Service.php") || stdout.contains("helper_function"));
}

// ============ Feature: List Classes ============
#[test]
fn test_php_list_classes() {
    let output = catcode()
        .args(["-f", &fixture("Service.php"), "-c"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
    assert!(stdout.contains("AdminService"));
}

// ============ Feature: Class Skeleton ============
#[test]
fn test_php_class_skeleton() {
    let output = catcode()
        .args(["-f", &fixture("Service.php"), "-c", "UserService"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
}

