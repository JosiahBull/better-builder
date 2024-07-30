#[test]
fn macro_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_tests/should_fail/error_on_enum.rs");
    t.compile_fail("tests/compile_tests/should_fail/error_on_tuple_struct.rs");
}

#[test]
fn macro_successes() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile_tests/should_pass/basic.rs");
    t.pass("tests/compile_tests/should_pass/all_optional.rs");
    t.pass("tests/compile_tests/should_pass/all_required.rs");
    t.pass("tests/compile_tests/should_pass/duplicates.rs");
    t.pass("tests/compile_tests/should_pass/function_collision.rs");
    t.pass("tests/compile_tests/should_pass/no_fields.rs");
    t.pass("tests/compile_tests/should_pass/trait_derivation.rs");
}
