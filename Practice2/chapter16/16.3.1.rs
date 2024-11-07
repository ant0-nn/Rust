fn main() {
	println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
	assert_eq!(format!("{1}{0}", 1, 2), "21");
	assert_eq!(format!("{0}{1}{0}", 2, 1), "212");
	println!("Success!");
}