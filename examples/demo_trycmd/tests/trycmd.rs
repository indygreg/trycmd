#[test]
fn trycmd() {
    trycmd_indygreg_fork::TestCases::new()
        .case("README.md")
        .insert_var("[REPLACEMENT]", "runtime-value")
        .unwrap();
}
