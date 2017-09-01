// Only library crates expose functions that other crates are able to call and use; binary crates are meant to be run on their own.

extern crate adder;

mod common;

#[test]
fn add_two_to_three(){
    use common::*;

    setup();
    assert_eq!(5, adder::add_two(3));
}