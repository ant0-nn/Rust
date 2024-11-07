fn main() {
    let color = String::from("green");

    let print = || println!("`color`: {}", color);

    print();
    print();

    let _reborrow = &color;

    println!("{}",color);
}