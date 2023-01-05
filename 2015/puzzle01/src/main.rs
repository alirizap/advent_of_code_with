use std::fs;


fn run(contents: String) {

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
    let contents = fs::read_to_string("1").expect("can not read file 1");
    run(contents);
}