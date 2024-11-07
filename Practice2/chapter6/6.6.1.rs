enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// Rust doesn't support floating-point numbers in C-like enums
// Enum variants must have integer values, so the Number2 definition is incorrect.
// We'll remove Number2 for now

fn main() {
    // To compare enum variants, we need to cast them to the same type (e.g., integer)
    assert_eq!(Number::One as i32, Number1::One as i32);

    println!("Success!");
}
