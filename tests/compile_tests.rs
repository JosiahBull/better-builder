#[test]
fn macro_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_tests/should_fail/error_on_enum.rs");
    t.compile_fail("tests/compile_tests/should_fail/error_on_tuple_struct.rs");
}
