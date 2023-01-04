use std::fs;


fn run(input: &str) {
    let contents = fs::read_to_string(input)
        .expect("unable to read input file");

    let mut floor = 0;
    let mut not_found = true;
    let mut position = 0;
    for (index, chr) in contents.chars().enumerate() {
        if floor == -1 && not_found {
            position = index;
            not_found = false;
        }
        if chr == '(' { floor += 1}
        else if chr == ')' { floor -= 1}
    }

    println!("Part One: {}", floor);
    println!("Part Two: {}", position);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).expect("filepath not provided");
    run(&input);
}