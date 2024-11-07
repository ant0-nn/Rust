fn main() {
    never_return();

    println!("Failed!"); 
}

fn never_return() -> ! {
    std::process::exit(1); 
}
