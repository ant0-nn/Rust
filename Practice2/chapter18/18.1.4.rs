fn main() {
    let example_closure_v1 = |x: String| -> String { x };
    let example_closure_v2 = |x: i32| -> i32 { x };
    
    let s = example_closure_v1(String::from("hello"));

    let n = example_closure_v2(5);
    println!("String: {}", s);
    println!("Integer: {}", n);
}