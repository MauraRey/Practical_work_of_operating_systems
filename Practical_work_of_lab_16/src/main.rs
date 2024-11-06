use itertools::Itertools;
use std::collections::HashMap;

fn find_values() -> Vec<HashMap<char, u8>> {
    let digits = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut results = Vec::new();

    for perm in digits.iter().permutations(8) {
        let mut map = HashMap::new();
        let letters = ['m', 'u', 'x', 'a', 's', 'l', 'o', 'n'];

        for (i, &ch) in letters.iter().enumerate() {
            map.insert(ch, *perm[i]);  // Розіменування *perm[i] для вставки значення
        }

        // Приведення результатів до u32
        let muxa = map[&'m'] as u32 * 1000 + map[&'u'] as u32 * 100 + map[&'x'] as u32 * 10 + map[&'a'] as u32;
        let x = map[&'x'] as u32;
        let a = map[&'a'] as u32;
        let slon = map[&'s'] as u32 * 1000 + map[&'l'] as u32 * 100 + map[&'o'] as u32 * 10 + map[&'n'] as u32;

        if muxa / x == slon {
            results.push(map);
        }
    }

    results
}

fn print_result(map: &HashMap<char, u8>) {
    let muxa = map[&'m'] as u32 * 1000 + map[&'u'] as u32 * 100 + map[&'x'] as u32 * 10 + map[&'a'] as u32;
    let x = map[&'x'] as u32;
    let a = map[&'a'] as u32;
    let slon = map[&'s'] as u32 * 1000 + map[&'l'] as u32 * 100 + map[&'o'] as u32 * 10 + map[&'n'] as u32;

    println!("muxa: {}", muxa);
    println!("x   a: {}    {}", x, a);
    println!("------");
    println!("slon: {}", slon);
}

fn main() {
    let results = find_values();

    for map in &results {
        print_result(&map);
    }

    println!("Total solutions: {}", results.len());
}
