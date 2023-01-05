use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point(i32, i32);

pub fn run(contents: &str) {
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
