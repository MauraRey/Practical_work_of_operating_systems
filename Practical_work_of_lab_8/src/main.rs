fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect()
}

fn main() {
    let test_cases = [
        ("Hello", "hELLO"),
        ("Привіт", "пРИВІТ"),
    ];

    for (a, b) in test_cases.iter() {
        println!("Original: {}, Inverted: {}", a, invert_the_case(a.to_string()));
        println!("Original: {}, Inverted: {}", b, invert_the_case(b.to_string()));
    }
}
