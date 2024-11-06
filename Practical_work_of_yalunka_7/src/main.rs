fn main() {
    let height = 5;

    (0..height).for_each(|i| {
        (0..(height - i - 1)).for_each(|_| print!(" "));
        (0..(2 * i + 1)).for_each(|_| print!("*"));
        println!();
    });

    (1..height).for_each(|i| {
        (0..(height - i - 1)).for_each(|_| print!(" "));
        (0..(2 * i + 1)).for_each(|_| print!("*"));
        println!();
    });

    (2..height).for_each(|i| {
        (0..(height - i - 1)).for_each(|_| print!(" "));
        (0..(2 * i + 1)).for_each(|_| print!("*"));
        println!();
    });

    println!("     *     ");
    println!("    ***    ");
    println!("   *****   ");
    println!("  *******  ");
    println!(" ********* ");
    println!("***********");
}
