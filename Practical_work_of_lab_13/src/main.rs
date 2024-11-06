fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();

    if total % n as u32 != 0 {
        return usize::MAX;
    }

    let average = total / n as u32;
    let mut moves = 0;
    let mut balance = 0;

    for &shipment in shipments.iter() {
        balance += shipment as i32 - average as i32;
        moves += balance.abs() as usize;
    }

    moves
}

fn main() {
    let shipments1 = vec![8, 2, 2, 4, 4];
    let shipments2 = vec![9, 3, 7, 2, 9];

    println!("Minimum moves for shipments1: {}", count_permutation(&shipments1));
    println!("Minimum moves for shipments2: {}", count_permutation(&shipments2));
}
