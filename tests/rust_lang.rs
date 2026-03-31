mod common;

fn file() -> String {
    common::fixture("rust", "sample.rs")
}

#[test]
fn lists_impl_point_with_fields() {
    let out = common::catcode(&[&file()]);
    assert!(out.contains("Impl Point"), "got:\n{out}");
    assert!(out.contains("field x"), "got:\n{out}");
    assert!(out.contains("field y"), "got:\n{out}");
}

#[test]
fn lists_impl_circle_with_fields() {
    let out = common::catcode(&[&file()]);
    assert!(out.contains("Impl Circle"), "got:\n{out}");
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
    assert!(out.contains("PI"), "got:\n{out}");
}

#[test]
fn method_of_finds_both() {
    let out = common::catcode(&[&file(), "-m", "of"]);
    assert_eq!(common::count_occurrences(&out, "Name:"), 2, "got:\n{out}");
}

#[test]
fn method_distance_shows_body() {
    let out = common::catcode(&[&file(), "-m", "distance"]);
    assert!(out.contains("sqrt"), "got:\n{out}");
}
