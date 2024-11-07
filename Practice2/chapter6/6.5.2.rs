struct Unit;
trait SomeTrait {
}

impl SomeTrait for Unit { }

fn main() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
} 

fn do_something_with_unit(u: Unit) {   } 
