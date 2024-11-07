fn main() {
    never_return();

    println!("Failed!"); 
}

fn never_return() -> ! {
    panic!("This function never returns!"); 
}
