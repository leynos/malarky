//! Compile-time visibility checks for the executable domain contract.

/// Checks public construction while rejecting access to private representation.
#[test]
fn domain_contract_visibility_is_enforced() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/ui/domain_contract/pass/*.rs");
    cases.compile_fail("tests/ui/domain_contract/fail/*.rs");
}
