use std::process::Command;

fn catcode() -> Command {
    Command::new(env!("CARGO_BIN_EXE_catcode"))
}

fn fixture(path: &str) -> String {
    format!("tests/ruby/{}", path)
}

// ============ Feature: List Methods (default) ============
#[test]
fn test_ruby_list_methods() {
    let output = catcode()
        .args(["-f", &fixture("service.rb")])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("create_user"));
    assert!(stdout.contains("get_user"));
    assert!(stdout.contains("delete_user"));
}

// ============ Feature: Method Detail ============
#[test]
fn test_ruby_method_detail() {
    let output = catcode()
        .args(["-f", &fixture("service.rb"), "-m", "create_user"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("create_user"));
    assert!(stdout.contains("L"));
}

// ============ Feature: List Classes ============
#[test]
fn test_ruby_list_classes() {
    let output = catcode()
        .args(["-f", &fixture("service.rb"), "-c"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
    assert!(stdout.contains("AdminService"));
}

// ============ Feature: Class Skeleton ============
#[test]
fn test_ruby_class_skeleton() {
    let output = catcode()
        .args(["-f", &fixture("service.rb"), "-c", "UserService"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
    assert!(stdout.contains("create_user"));
}

