use std::fs;


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

fn not_disallowed_substrings(word: &str) -> bool {
    !((word.contains("ab") || word.contains("cd") || word.contains("pq") || word.contains("xy")))
}

fn is_nice(line: &str) -> bool {
    vowels_check(line) && twice_letter(line) && not_disallowed_substrings(line)
}

fn run(lines: Vec<&str>) {
    let mut count = 0;
    for line in lines {
        if is_nice(line) {
            count += 1;
        }
    }

    println!("Part One: {}", count);
}

fn main() {
    let contents = fs::read_to_string("5").expect("can not read file 5");
    let lines: Vec<&str> = contents.split('\n')
                        .map(|line| line.trim())
                        .collect();

    assert!(vowels_check("xazegov"));
    assert!(vowels_check("aeiouaeiouaeiou"));
    assert!(twice_letter("abcdde"));
    assert!(twice_letter("aa"));
    assert!(!not_disallowed_substrings("ab"));
    assert!(!is_nice("jchzalrnumimnmhp"));
    assert!(!is_nice("haegwjzuvuyypxyu"));
    assert!(!is_nice("dvszwmarrgswjxmb"));
    assert!(is_nice("aaa"));
    assert!(is_nice("ugknbfddgicrmopn"));
    run(lines);
}
