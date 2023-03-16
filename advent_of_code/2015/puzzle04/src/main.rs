

fn compute(input: String, word: &str) -> usize {
    let mut num: usize = 0;
    loop {
        let tmp = input.clone() + &num.to_string();
        let digest = md5::compute(tmp.as_bytes());
        let value = format!("{:x}", digest);
        if value.starts_with(word) {
            break;
        }
        num += 1;
    }
    return num;
}

fn solve(input: String) {
    println!("Part One: {}", compute(input.clone(), "00000"));
    println!("Part Two: {}", compute(input.clone(), "000000"))
}

fn main() {
    let input = std::env::args().nth(1).expect("input not found!");
    solve(input);
}