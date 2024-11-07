struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let v = Point(0, 127, 255);
    check_color(Color(v.0, v.1, v.2));

    println!("Success!");
}   

fn check_color(p: Color) {
    let Color(x, y, z) = p;
    assert_eq!(x, 0);
    assert_eq!(y, 127);
    assert_eq!(z, 255);
}
