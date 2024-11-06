use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_index = (0, 0);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = (i, i + 1);
        }
    }

    (min_sum, min_index.0, min_index.1)
}

fn main() {
    let data = gen_random_vector(20);
    println!("Data: {:?}", data);

    let (min_sum, index1, index2) = min_adjacent_sum(&data);
    println!(
        "Min adjacent sum = {} at indexes: {}, {}",
        min_sum, index1, index2
    );
}
