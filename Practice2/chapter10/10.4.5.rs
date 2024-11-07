trait MyTrait {
    fn f(&self) -> MyEnum;
}

enum MyEnum {
    U32(u32),
    String(String),
}

impl MyTrait for u32 {
    fn f(&self) -> MyEnum { MyEnum::U32(42) }
}

impl MyTrait for String {
    fn f(&self) -> MyEnum { MyEnum::String(self.clone()) }
}

fn my_function(x: Box<dyn MyTrait>)  {
    let _ = x.f();
}

fn main() {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));

    println!("Success!");
}
