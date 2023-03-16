

fn string_size(line: &str) -> usize {

    let mut size: usize = 0;
    let mut offset = 0;
    let mut backslash = false;

    for ch in line.chars() {
        match ch {
            '\\' | '\"' if backslash  => { size += 1; backslash=false}
            'x' if backslash => { offset += 2; size += 1; backslash=false}
            '\"' if !backslash => (),
            '\\' => { backslash=true }
            _ => size += 1           
        }
    }
    size.saturating_sub(offset)
}

fn encode_string_size(line: &str) -> usize {
    
    let mut size = line.len() + 2;
    for ch in line.chars() {
        match ch {
            '\\' | '\"' => size += 1,
            _ => ()
        }
    }
    size
}

fn run(lines: Vec<&str>) {
    
    let mut chars_num_literal = 0;
    let mut chars_num_memory = 0;
    let mut chars_encode = 0;

    for line in lines {
        chars_num_literal += line.len();
        chars_num_memory += string_size(line);
        chars_encode += encode_string_size(line);
    }

    println!("Part One: {}", chars_num_literal - chars_num_memory);
    println!("Part Two: {}", chars_encode - chars_num_literal);
}


fn main() {
    let contents = std::fs::read_to_string("8")    
        .expect("can not read file 8");

    let lines: Vec<&str> = contents.split('\n')
        .map(|line| line.trim())
        .collect();

    run(lines);

}
