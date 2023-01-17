use regex::Regex;

fn run(input: String) {
    let re = Regex::new(r"-?[0-9]+").unwrap();
    let sum: i32 = re.find_iter(&input)
        .filter_map(|num| num.as_str().parse().ok())
        .collect::<Vec<i32>>()
        .iter().sum();

    println!("Part One: {}", sum);
}

fn main() {
    let contents = std::fs::read_to_string("12").expect("can not read file 12");
    run(contents);
}
