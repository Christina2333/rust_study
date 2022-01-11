mod common;

extern crate rust_study;

use rust_study::cp11_test::adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add2(2));
    common::setup();
}

// cargo test --test integration_test 执行单元测试
// 可以通过创建common文件夹，来存放tests中可以公共使用的部分