use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    let mut min_sum = i32::MAX;
    let mut min_indices = (0, 1);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_indices = (i, i + 1);
        }
    }

    (min_indices.0, min_indices.1, min_sum)
}

fn display_result(data: &[i32], min_i: usize, min_j: usize, min_sum: i32) {
    print!("indexes:");
    for i in 0..data.len() {
        print!("{:>4}.", i);
    }
    println!();

    print!("data:  [");
    for (i, &val) in data.iter().enumerate() {
        print!("{}", val);
        if i != data.len() - 1 {
            print!(", ");
        }
    }
    println!("]");

    print!("indexes:");
    for i in 0..data.len() {
        if i == min_i {
            print!("\\__");
        } else if i == min_j {
            print!(" __/");
        } else {
            print!("     ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[min_i], data[min_j], min_sum, min_i, min_j
    );
}

fn main() {
    let data = gen_random_vector(20);
    let (min_i, min_j, min_sum) = min_adjacent_sum(&data);
    display_result(&data, min_i, min_j, min_sum);
}