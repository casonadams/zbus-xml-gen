#[allow(dead_code)]
pub fn assert_contains(actual: &str, expected: &str) {
    if !actual.contains(expected) {
        panic!(
            "Generated output did not contain expected substring!\n---EXPECTED---\n{}\n---ACTUAL---\n{}\n",
            expected, actual
        );
    }
}

#[allow(dead_code)]
pub fn assert_not_contains(actual: &str, not_expected: &str) {
    if actual.contains(not_expected) {
        panic!(
            "Generated output should NOT contain substring!\n---NOT EXPECTED---\n{}\n---ACTUAL---\n{}\n",
            not_expected, actual
        );
    }
}
