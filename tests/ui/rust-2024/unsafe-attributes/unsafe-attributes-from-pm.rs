// Test for unsafe attributes generated by a proc-macro.
// See https://github.com/rust-lang/rust/issues/132906

//@ revisions: edition2021 edition2024
//@[edition2021] check-pass
//@[edition2021] edition:2021
//@[edition2024] edition:2024
//@[edition2024] compile-flags: -Zunstable-options
//@ aux-crate: unsafe_attributes_pm=unsafe-attributes-pm.rs

unsafe_attributes_pm::missing_unsafe!();

unsafe_attributes_pm::macro_rules_missing_unsafe!();
//[edition2024]~^ ERROR unsafe attribute used without unsafe

make_fn!();

fn main() {}
