mod config;

use rust_tests;

#[test]
fn it_works() {
    config::setup();
    let result = rust_tests::add(2, 2);
    assert_eq!(result, 4);
}
