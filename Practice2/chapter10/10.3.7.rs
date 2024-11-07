use std::ops::Add;

fn sum<T>(x: T, y: T) -> T
where
    T: Add<Output = T>,
{
    x + y
}

fn main() {
    assert_eq!(sum(1, 2), 3);
}
