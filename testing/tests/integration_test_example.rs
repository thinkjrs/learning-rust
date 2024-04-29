extern crate test_lib;
use test_lib::adder;
mod common;

#[test]
fn it_adds_two_integration() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
