fn main() {
let layers = 10;
let numbers = 5;

for layer in 0..layers {
let cols = numbers + layer * 2;
let rows = (cols + 1) / 2;

let max_cols = numbers + (layers - 1) * 2;
let spaces_before = (max_cols - cols) / 2;

for row in 0..rows {
for _ in 0..spaces_before {
print!(" ");
}
for col in 0..cols {
if col >= (cols / 2) - row && col <= (cols / 2) + row {
print!("*");
} else {
print!(" ");
}
}
println!();
}
}
}