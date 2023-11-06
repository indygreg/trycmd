#[test]
#[cfg(feature = "examples")]
fn example_tests() {
    let t = trycmd_indygreg_fork::TestCases::new();
    t.register_bins(trycmd_indygreg_fork::cargo::compile_examples([]).unwrap());
    t.case("examples/*.md");
}
