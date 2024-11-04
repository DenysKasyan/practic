use rand::Rng;

#[test]
fn test_min_sum() {
    let random_vec = create_random_vector(20);
    println!("{:?}", random_vec);
    println!("min_adjacent_sum: {}", find_min_adjacent_sum(&random_vec));
}

fn create_random_vector(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(10..100)).collect()
}

fn find_min_adjacent_sum(array: &[i32]) -> i32 {
    array.windows(2)
        .map(|pair| pair[0] + pair[1])
        .min()
        .unwrap()
}