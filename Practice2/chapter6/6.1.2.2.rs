fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(s);
}

fn greetings(s: Box<str>) {
    println!("{}", s);
}
