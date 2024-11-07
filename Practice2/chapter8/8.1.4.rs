fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    for ab in alphabets {
        if matches!(ab, 'a'..='z' | 'A'..='Z') {
            println!("'{}' is a letter", ab);
        } else {
            println!("'{}' is not a letter", ab);
        }
    }

    println!("Success!");
}
