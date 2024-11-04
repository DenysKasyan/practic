#[test]
fn test_count_permutation() {
    let shipments = vec![9, 3, 7, 2, 9];
    println!("{}", calculate_moves(&shipments));
}

fn calculate_moves(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let count = shipments.len() as u32;

    if total % count != 0 {
        panic!("Error");
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
