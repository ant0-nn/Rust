enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    let _x = Operations::Add; 

    let operation = Operations::Subtract; 
    match operation {
        Operations::Add => println!("Adding"),
        Operations::Subtract => println!("Subtracting"),
    }
}
