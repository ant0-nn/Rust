use std::ops::Add;
use std::fmt;

struct Meters(u32);

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There are still {} meters left", self.0)
    }
}

impl Add for Meters {
    type Output = Self;

    fn add(self, other: Meters) -> Self {
        Self(self.0 + other.0)
    }
}

fn calculate_distance(m1: Meters, m2: Meters) -> Meters {
    m1 + m2 
}

fn main() {
    let d = calculate_distance(Meters(10), Meters(20));
    assert_eq!(format!("{}", d), "There are still 30 meters left");

    println!("Success!"); 
}
