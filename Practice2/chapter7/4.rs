fn main() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    for name in &names {
        // Do something with name...
        println!("{}", name);
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    for n in numbers {
        // Do something with n...
        println!("{}", n);
    }

    println!("{:?}", numbers);
}
