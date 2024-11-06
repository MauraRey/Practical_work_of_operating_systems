fn main() {
    let n = 5;

    for i in 0..n {
        for _ in 0..(n - i - 1) {
            print!(" ");
        }
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }

    for i in (0..n - 1).rev() {
        for _ in 0..(n - i - 1) {
            print!(" ");
        }
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}
