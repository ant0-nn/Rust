fn main() {
    let mut s = String::new();
    s.push_str("hello");

    // Some bytes, in a vector
    let v = vec![104, 101, 108, 108, 111];

    // Turn a byte's vector into a String
    let s1 = String::from_utf8(v).expect("Found invalid UTF-8");

    assert_eq!(s, s1);

    println!("Success!");
}
