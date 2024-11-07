fn main() {
    let mut s = String::from("hello, ");

    // let r1 = &mut s; 
    s.push_str("world"); 
    let r2 = &mut s; 
    r2.push_str("!");

    println!("{}", r2); 
}
