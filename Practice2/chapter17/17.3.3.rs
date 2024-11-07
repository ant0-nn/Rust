fn f<'a, 'b>(x: &'a i32, y: &'b mut &'a i32) {
    *y = x;
}

fn main() {
    let x = 10;
    let mut y = &0;
    
    f(&x, &mut y);
    println!("y: {}", y);
}