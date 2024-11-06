fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let numbers = [1, 10007, 92, 4, 60, 16, 45, 37, 28, 5];

    for &num in numbers.iter() {
        println!("Is {} prime? {}", num, is_prime(num));
    }
}
