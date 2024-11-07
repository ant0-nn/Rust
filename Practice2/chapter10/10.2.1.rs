struct Array<T, const N: usize> {
    data: [T; N],
}

fn main() {
    let arrays: [Array<i32, 3>; 2] = [
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3],
        },
    ];

    let arrays_f32: [Array<f32, 3>; 1] = [
        Array {
            data: [1.0, 2.0, 3.0],
        },
    ];

    let arrays_i32: [Array<i32, 2>; 1] = [
        Array {
            data: [1, 2],
        },
    ];

    println!("Success!");
}
