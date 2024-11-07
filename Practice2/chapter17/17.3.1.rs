struct DoubleRef<'r, 's, T> 
where 's: 'r,
{
    r: &'r T,
    s: &'s T,
}

fn main() {
    println!("Success!");
}