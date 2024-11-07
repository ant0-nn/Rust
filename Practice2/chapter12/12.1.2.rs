fn main() {
    assert_eq!(u8::MAX, 255);
 
    let v = (1000u16 % 256) as u8; 

    println!("Success!");
}
