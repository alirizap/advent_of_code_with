fn find_pair(bytes: &[u8], last_idx: usize, except: Option<u8>) -> Option<u8> {
    for i in 1..last_idx {
        if bytes[i] == bytes[i - 1] {
            match except {
                Some(v) if v != bytes[i] => return Some(bytes[i]),
                None => return Some(bytes[i]),
                _ => (),
            }
        }
    }
    None
}

fn invalid_letters(bytes: &[u8]) -> bool {
    bytes.iter().any(|&x| x == b'i' || x == b'l' || x == b'o')
}

fn has_inc_letters(bytes: &[u8], last_idx: usize) -> bool {
    for i in 0..last_idx - 2 {
        if bytes[i] + 1 == bytes[i + 1] && bytes[i + 1] + 1 == bytes[i + 2] {
            return true;
        }
    }
    false
}

fn increment_password(bytes: &mut [u8], last_idx: usize) {
    let mut idx = last_idx - 1;
    loop {
        if bytes[idx] == b'z' {
            bytes[idx] = b'a';
            idx -= 1;
        } else {
            bytes[idx] += 1;
            break;
        }
    }
}

fn new_password(password: String) -> String {
    let mut bytes = password.bytes().collect::<Vec<u8>>();
    let last_idx = bytes.len();

    loop {
        increment_password(&mut bytes, last_idx);

        if invalid_letters(&bytes) {
            continue;
        }

        if !has_inc_letters(&bytes, last_idx) {
            continue;
        }

        let first_pair = find_pair(&bytes, last_idx, None);
        if first_pair.is_none() {
            continue;
        }
        let second_pair = find_pair(&bytes, last_idx, first_pair);
        if second_pair.is_some() {
            break;
        }
    }

    String::from_utf8_lossy(&bytes).to_string()
}

fn run(password: String) {
    let password_1 = new_password(password);
    println!("Part One: {}", password_1);
    let password_2 = new_password(password_1);
    println!("Part Two: {}", password_2);
}

fn main() {
    let input = std::env::args().nth(1).expect("No Input");
    run(input);
}
