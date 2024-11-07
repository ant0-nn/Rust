use std::fmt::Display;

fn foobar<T: Display>(thing: T) {
    println!("{}", thing); 
}

fn main() {
    foobar(42); 
    foobar("Hello, World!"); 
}
