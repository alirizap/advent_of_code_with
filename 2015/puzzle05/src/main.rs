use std::fs;

// Part One Rule
fn vowels_check(word: &str) -> bool {
    let mut vowels_count = 0;
    for letter in word.chars() {
        match letter {
            'a' => vowels_count += 1,
            'e' => vowels_count += 1,
            'i' => vowels_count += 1,
            'o' => vowels_count += 1,
            'u' => vowels_count += 1,
            _ => ()
        }
        if vowels_count >= 3 {
            return true;
        }
    }

    false 
}

// Part One Rule 
fn twice_letter(word: &str) -> bool {
    let len = word.len();
    let bytes: Vec<u8> = word.bytes().collect();
    for i in 1..len {
        if bytes[i-1] == bytes[i] {
            return true;
        }
    }
    false
}

// Part One Rule
fn not_disallowed_substrings(word: &str) -> bool {
    !((word.contains("ab") || word.contains("cd") || word.contains("pq") || word.contains("xy")))
}

fn is_nice_1(line: &str) -> bool {
    vowels_check(line) && twice_letter(line) && not_disallowed_substrings(line)
}

// Part Two Rule
fn twice_pair(word: &str) -> bool {
    let len = word.len();
    for i in 0..len-1 {
        match word.get(i..i+2) {
            Some(sub) => {
                if let Some(x) = word.get(i+2..len) {
                    if x.contains(&sub) {
                        return true;
                    }
                }
            }
            None => (),
        }
    }
    false 
}

// Part Two Rule
fn letter_repeat(word: &str) -> bool {
    let len = word.len();
    let bytes: Vec<u8> = word.bytes().collect();
    for i in 2..len {
        if bytes[i] == bytes[i-2] {
            return true;
        }
    }
    false 
}

fn is_nice_2(line: &str) -> bool {
    twice_pair(line) && letter_repeat(line)
}

fn run(lines: &Vec<&str>) {
    let mut count_1 = 0;
    let mut count_2 = 0;
    for line in lines {
        if is_nice_1(line) {
            count_1 += 1;
        }

        if is_nice_2(line) {
            count_2 += 1;
        }
    }
    println!("Part One: {}", count_1);
    println!("Part Two: {}", count_2);
}

fn main() {
    let contents = fs::read_to_string("5").expect("can not read file 5");
    let lines: Vec<&str> = contents.split('\n')
                        .map(|line| line.trim())
                        .collect();

    assert!(!is_nice_1("jchzalrnumimnmhp"));
    assert!(!is_nice_1("haegwjzuvuyypxyu"));
    assert!(!is_nice_1("dvszwmarrgswjxmb"));
    assert!(is_nice_1("aaa"));
    assert!(is_nice_1("ugknbfddgicrmopn"));
    assert!(is_nice_2("qjhvhtzxzqqjkmpb"));
    assert!(is_nice_2("xxyxx"));
    assert!(!is_nice_2("uurcxstgmygtbstg"));
    assert!(!is_nice_2("ieodomkazucvgmuy"));

    run(&lines);
}
