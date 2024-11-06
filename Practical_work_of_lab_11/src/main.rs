fn is_palindrome(x: u32) -> bool {
    let original = x;
    let mut reversed = 0;
    let mut num = x;

    while num > 0 {
        reversed = reversed * 10 + num % 10;
        num /= 10;
    }

    original == reversed
}

fn main() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
    ];

    for (n, exp) in data.iter() {
        println!("Is {} a palindrome? {}", n, is_palindrome(*n));
        assert_eq!(is_palindrome(*n), *exp);
    }
}
