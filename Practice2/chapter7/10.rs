
// Fill in the blank
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 20 {
            break counter;
        }
    };

    assert_eq!(result, 20);
    println!("Success!");
}