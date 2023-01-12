use std::env;

fn get_length(rng: usize, num: String) -> usize {
    let mut chars: Vec<char> = num.chars().collect();
    let mut counter = 0;
    let length = loop {
        let mut tmp = Box::new(String::new());
        let mut prev = chars[0];
        let mut count = 0;
        for ch in chars.iter() {
            if *ch == prev {
                count += 1;
                prev = *ch;
            } else {
                tmp.push_str(&format!("{}{}", count, prev));
                count = 1;
                prev = *ch;
            }
        }
        tmp.push_str(&format!("{}{}", count, prev));
        chars.clear();
        counter += 1;
        if counter == rng {
            break tmp.len();
        }
        chars = tmp.chars().collect();
    };

    length
}

fn run(num: String) {
    let part_1 = get_length(40, num.clone());
    let part_2 = get_length(50, num);

    println!("Part One: {}", part_1);
    println!("Part Two: {}", part_2);
}

fn main() {
    let number: String = env::args().nth(1).expect("input not provieded");
    run(number);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_40_times() {
        assert_eq!(252594, get_length(40, "1113222113".to_string()));
    }

    #[test]
    fn test_50_times() {
        assert_eq!(3579328, get_length(50, "1113222113".to_string()))
    }
}
