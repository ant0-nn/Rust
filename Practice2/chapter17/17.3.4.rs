fn call_on_ref_zero<F>(f: F) 
where F: for<'a> Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}

fn main() {
    call_on_ref_zero(|x| println!("x: {}", x));
    println!("Success!");
}
