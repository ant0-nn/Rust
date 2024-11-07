#![allow(incomplete_features)]

fn check_size_u8(val: [u8; 767]) {}
fn check_size_i32(val: [i32; 191]) {}
fn check_size_str(val: [&str; 1]) {}
fn check_size_string(val: Vec<String>) {}
fn check_size_char(val: [char; 1]) {}

fn main() {
    check_size_u8([0u8; 767]);
    check_size_i32([0i32; 191]);
    check_size_str(["hello你好"; 1]);
    check_size_string(vec!["hello你好".to_string(); 192]); 
    check_size_char(['中'; 1]);

    println!("Success!");
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}
