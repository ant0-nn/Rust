fn is_palindrome(x: u32) -> bool {
    let original = x.to_string();
    let reversed: String = original.chars().rev().collect();

    original == reversed
}

#[test]
fn test() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
    ];

    data.iter().for_each(|(n, exp)| {
        assert_eq!(is_palindrome(*n), *exp);
    });
}

fn main() {
    let number = 1221;
    println!("Is {} a palindrome? {}", number, is_palindrome(number));
}