use std::fmt;

#[derive(Debug)]
struct Structure(i32);

struct Deep(Structure);

impl fmt::Debug for Deep {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "{}", (self.0).0)
	}
}

fn main() {
	println!("Now {:?} will print!", Deep(Structure(7)));
}
