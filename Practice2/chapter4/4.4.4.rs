fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => Some(42), 
        _ => None,     
    };
    
    never_return_fn()
}

// Спосіб 1: Використання panic!
fn never_return_fn() -> ! {
    panic!("This function never returns!"); 
}

// Спосіб 2: Використання std::process::exit()
// fn never_return_fn() -> ! {
//     std::process::exit(1); 
// }

// Спосіб 3: Безкінечний цикл
// fn never_return_fn() -> ! {
//     loop {
//     }
// }
