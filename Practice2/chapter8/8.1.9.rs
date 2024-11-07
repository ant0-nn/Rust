fn main() {
    let age = Some(30);
    
    if let Some(age) = age { 
        assert_eq!(age, 30); 
    } 
    
    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, its value is {}", age),
        _ => ()
    }
}
