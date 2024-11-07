use std::ops::Sub;

#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 2.0, y: 3.0 } - Point { x: 1.0, y: 0.0 },
        Point { x: 1.0, y: 3.0 });

    println!("Success!");
}
