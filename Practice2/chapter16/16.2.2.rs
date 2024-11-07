use std::fmt;

struct Person {
	name: String,
	age: u8,
}

impl fmt::Display for Person {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "Person {{\n	name: \"{}\",\n	age: {},\n}}", self.name, self.age)
	}
}

fn main() {
	let person = Person {
    	name: "Sunface".to_string(),
    	age: 18,
	};

	println!("{}", person);
}