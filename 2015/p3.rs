use std::collections::HashSet;
use std::{fs, env};


#[derive(Hash, Eq, PartialEq, Debug)]
struct Point(i32, i32);

fn run(input: &str) {
    let contents = fs::read_to_string(input).expect("cannot read file");
    let mut houses = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    houses.insert(Point(x, y));
    
    for dir in contents.trim().chars() {
        match dir {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => unreachable!()
        }
        houses.insert(Point(x, y));
    }

    println!("Part One: {}", houses.len())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args.get(1).expect("file or input not provided");
    run(&input);
}