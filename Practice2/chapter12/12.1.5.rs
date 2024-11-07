fn main() {
    let arr: [u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64; 13] = &arr;
    let b = a as *const [u8; 104]; // 13 * 8 = 104
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), 104);
    }

    println!("Success!");
}
