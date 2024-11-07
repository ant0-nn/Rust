fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3); // Заповнюємо пропуск

    // Integer subtraction
    assert!(1i32 - 2 == -1); // Заповнюємо пропуск
    assert!(1u8 as i32 - 2 == -1); // Необхідно привести тип

    assert!(3 * 50 == 150); // Заповнюємо пропуск

    // Use epsilon to compare floating-point values
    let epsilon = 1e-10; // Похибка
    assert!((9.6f64 / 3.2f64 - 3.0f64).abs() < epsilon); // Порівнюємо з похибкою

    assert!(24 % 5 == 4); // Заповнюємо пропуск
    // Short-circuiting boolean logic
    assert!(true && false == false); // Заповнюємо пропуск
    assert!(true || false == true); // Заповнюємо пропуск
    assert!(!true == false); // Заповнюємо пропуск

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
