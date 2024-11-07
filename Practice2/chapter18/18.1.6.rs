fn main() {
    let mut s = String::new();

    let update_string = |str: &str| s.push_str(str);

    exec(update_string);

    println!("{:?}", s);
}

fn exec<F>(mut f: F)
where
    F: FnMut(&str),
{
    f("hello");
}
