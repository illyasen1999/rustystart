// Rust knows where to look for integration tests inside the tests directory and turns each file as a crate
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two_v2(2));
}

// https://youtu.be/-L4nKAlmH3M?t=819