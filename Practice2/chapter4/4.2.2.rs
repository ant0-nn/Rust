fn main() {
    let c1 = "中"; 
    if let Some(c) = c1.chars().next() { 
        print_char(c);
    } else {
        println!("No character found."); 
    }
}

fn print_char(c: char) {
    println!("{}", c);
}
