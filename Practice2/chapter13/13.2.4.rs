use std::num::ParseIntError;

fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
    // Спосіб 1: Використання `map`
    n_str.parse::<i32>().map(|n| n + 2)

    // Спосіб 2: Використання `and_then`
    // n_str.parse::<i32>().and_then(|n| Ok(n + 2))
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!");
}
