fn valid_output_owning() -> String { 
    String::from("foo")  
}

fn main() {
    let s = valid_output_owning();
    println!("{}", s);
}
