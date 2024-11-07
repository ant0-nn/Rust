fn main() {
    let s = "Hello, world!";

    println!("{0:.5}", s);

    assert_eq!(format!("Hello {:.3}!", "abcdefg"), "Hello abc!");

    println!("Success!");
}
