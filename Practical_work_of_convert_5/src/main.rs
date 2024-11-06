fn main() {
    let n = 22;

    for i in 0..n {
        for j in 0..n {
            if i == 0 || i == n - 1 || j == 0 || j == n - 1 || i == j || i + j == n - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
