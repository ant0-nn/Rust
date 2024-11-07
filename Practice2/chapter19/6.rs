enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            VeryVerboseEnumOfThingsToDoWithNumbers::Add => x + y,
            VeryVerboseEnumOfThingsToDoWithNumbers::Subtract => x - y,
        }
    }
}

fn main() {
    let add_operation = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
    let subtract_operation = VeryVerboseEnumOfThingsToDoWithNumbers::Subtract;

    let sum = add_operation.run(5, 3);
    let difference = subtract_operation.run(5, 3);

    println!("Sum: {}", sum);           
    println!("Difference: {}", difference); 
}
