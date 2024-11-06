struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut grid = vec![vec![false; 15]; 15];
    let mut occupied = 0;

    for rect in xs.iter() {
        for x in rect.a.x..rect.b.x {
            for y in rect.b.y..rect.a.y {
                if !grid[x as usize][y as usize] {
                    grid[x as usize][y as usize] = true;
                    occupied += 1;
                }
            }
        }
    }

    occupied
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    area_occupied_test();
    println!("Test passed!");
}
