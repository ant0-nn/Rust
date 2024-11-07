fn main() {
    let epsilon = 1e-10; 
    assert!((0.1 + 0.2 - 0.3).abs() < epsilon);

    println!("Success!");
}
