fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s; 
        println!("{}", r1); 
    }
    let r2 = &s; 

    println!("{}", r2); 

    println!("Success!");
}

