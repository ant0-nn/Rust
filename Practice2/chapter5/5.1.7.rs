fn main() {
    let mut x = Box::new(5); 
    
    let y = &mut *x; 
    
    *y = 4; 
    
    assert_eq!(*x, 4); 

    println!("Success!");
}
