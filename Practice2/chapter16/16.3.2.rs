fn main() {
	println!("{argument}", argument = "test");

	assert_eq!(format!("{name}{}", 1, name = "2"), "21");
	assert_eq!(format!("{a} {c} {b}", a = "a", b = 'b', c = 3), "a 3 b");

	println!("{0} {abc}", 2, abc = "def");

	println!("Success!");
}
