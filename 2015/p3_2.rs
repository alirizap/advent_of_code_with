use std::collections::HashSet;
use std::{fs, env};


#[derive(Hash, Eq, PartialEq, Debug)]
struct Point(i32, i32);

fn is_odd(num: usize) -> bool {
    return num % 2 != 0
}

fn run(input: &str) {
    let contents = fs::read_to_string(input).expect("cannot read file");
    // let contents = "^>v<".to_string();
    let mut houses = HashSet::new();
    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;
    houses.insert(Point(santa_x, santa_y));
    for (index, dir) in contents.trim().chars().enumerate() {
        match dir {
            '^' => { if is_odd(index) { robo_y += 1} else { santa_y += 1 } },
            'v' => { if is_odd(index) { robo_y -= 1} else { santa_y -= 1 } },
            '>' => { if is_odd(index) { robo_x += 1} else { santa_x += 1 } },
            '<' => { if is_odd(index) { robo_x -= 1} else { santa_x -= 1 } },
            _ => unreachable!()
        }
        houses.insert(Point(santa_x, santa_y));
        houses.insert(Point(robo_x, robo_y));
    }

    println!("Part Two: {}", houses.len())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args.get(1).expect("file or input not provided");
    run(&input);
}