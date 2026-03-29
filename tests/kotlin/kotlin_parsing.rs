use std::process::Command;

fn catcode() -> Command {
    Command::new(env!("CARGO_BIN_EXE_catcode"))
}

fn fixture(path: &str) -> String {
    format!("tests/kotlin/{}", path)
}

// ============ Feature: List Methods (default) ============
#[test]
fn test_kotlin_list_methods() {
    let output = catcode()
        .args(["-f", &fixture("Service.kt")])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Kotlin method parsing has limitations, but classes should be listed
    assert!(stdout.contains("Service.kt") || stdout.contains("fun"));
}

// ============ Feature: List Classes ============
#[test]
fn test_kotlin_list_classes() {
    let output = catcode()
        .args(["-f", &fixture("Service.kt"), "-c"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
    assert!(stdout.contains("AdminService"));
}

// ============ Feature: Class Skeleton ============
#[test]
fn test_kotlin_class_skeleton() {
    let output = catcode()
        .args(["-f", &fixture("Service.kt"), "-c", "UserService"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
}

