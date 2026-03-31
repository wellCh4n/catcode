mod common;

fn file() -> String {
    common::fixture("cpp", "sample.cpp")
}

#[test]
fn lists_classes_and_fields() {
    let out = common::catcode(&[&file()]);
    assert!(out.contains("Class Point"), "got:\n{out}");
    assert!(out.contains("field x"), "got:\n{out}");
    assert!(out.contains("field y"), "got:\n{out}");
    assert!(out.contains("Class Circle"), "got:\n{out}");
    assert!(out.contains("field radius"), "got:\n{out}");
}

#[test]
fn lists_fn_distance_and_area() {
    let out = common::catcode(&[&file()]);
    assert!(out.contains("fn distance"), "got:\n{out}");
    assert!(out.contains("fn area"), "got:\n{out}");
}

#[test]
fn method_area_shows_body() {
    let out = common::catcode(&[&file(), "-m", "area"]);
    assert!(out.contains("3.14159"), "got:\n{out}");
}

#[test]
fn method_of_finds_both() {
    let out = common::catcode(&[&file(), "-m", "of"]);
    assert_eq!(common::count_occurrences(&out, "Name:"), 2, "got:\n{out}");
}

#[test]
fn scoped_circle_of_returns_one() {
    let out = common::catcode(&[&file(), "-c", "Circle", "-m", "of"]);
    assert_eq!(common::count_occurrences(&out, "Name:"), 1, "got:\n{out}");
    assert!(out.contains("radius"), "got:\n{out}");
}

#[test]
fn class_lookup_shows_full_body() {
    let out = common::catcode(&[&file(), "-c", "Point"]);
    assert!(out.contains("distance"), "got:\n{out}");
}
