fn main() {
    let count = 0;

    let inc = || {
        println!("`count`: {}", count);
    };

    inc();


    let _reborrow = &count; 

    inc();

    let _count_reborrowed = &mut {count}; 

    assert_eq!(count, 0);
}
fn main() {
    let count = 0;

    let inc = || {
        println!("`count`: {}", count);
    };

    inc();


    let _reborrow = &count; 

    inc();

    let _count_reborrowed = &mut {count}; 

    assert_eq!(count, 0);
}
