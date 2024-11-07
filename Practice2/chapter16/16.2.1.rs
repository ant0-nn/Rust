struct Structure(i32);

fn main() {
	println!("{} months in a year.", 12);

	println!("Now {:?} will print!", Structure(3));
}

use std::fmt;

impl fmt::Debug for Structure {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "Structure({})", self.0)
	}
}