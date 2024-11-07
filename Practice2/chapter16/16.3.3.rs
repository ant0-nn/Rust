fn main() {
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);

    assert_eq!(format!("Hello {:width$}!", "x", width = 5), "Hello x    !");
    assert_eq!(format!("Hello {:5}!", "x"), "Hello x    !");

    println!("Success!");
}
