use rand::Rng;

#[test]
fn test_count_permutation() {
    let shipments = vec![9, 3, 7, 2, 9];
    println!("{}", calculate_moves(&shipments));
}

fn calculate_moves(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let count = shipments.len() as u32;

    if total % count != 0 {
        panic!("Неможливо розподілити рівномірно");
    }

    let average = total / count;
    let mut total_moves = 0;
    let mut current_balance: i32 = 0;

    for &shipment in shipments {
        current_balance += (shipment as i32) - (average as i32);
        total_moves += current_balance.abs() as usize;
    }

    total_moves
}


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