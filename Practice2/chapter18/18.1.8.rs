fn main() {
    let mut s = String::new();

    let update_string = |str: &str| { s.push_str(str); s.clone() };

    exec(update_string);
}

fn exec<F: FnMut(&str) -> String>(mut f: F) {
    println!("{}", f("hello"));
}