use std::collections::HashSet;

#[derive(Debug)]
struct Code { key: String, value: String}

impl From<String> for Code {
    fn from(line: String) -> Self {
        let splited_line = line.split(" => ").collect::<Vec<&str>>();
        Self {
            key: splited_line[0].to_string(),
            value: splited_line[1].to_string(),
        }
    }
}

fn replacement(key: &str, value: &str, s: String) -> Vec<String> {
    let key_len = key.len();
    let indices: Vec<_> = s.match_indices(key).collect();
    let mut result: Vec<String> = vec![];
    for (i, _) in indices {
        let mut new_s = s.clone();
        new_s.replace_range(i..i+key_len, value);
        result.push(new_s);
    }
    result
}


fn run(lines: Vec<String>) {
    let mut codes: Vec<Code> = vec![];
    let mut medicine_molecule = String::new();
    
    for line in lines {
        if line.contains("=>") {
            codes.push(line.into());
        }else if !line.is_empty() {
            medicine_molecule = line;
        }
    }

    let mut distincts = HashSet::new();
    for code in &codes {
        let result = replacement(code.key.as_str(), code.value.as_str(), medicine_molecule.clone());

        for r in result {
            distincts.insert(r);
        }
    }

    println!("Parts One: {}", distincts.len());

    let target = "e";
    let mut value = medicine_molecule;
    let mut steps = 0;
    while value != target {
        for code in &codes {
            if value.contains(&code.value) {
                value = value.replacen(code.value.as_str(), code.key.as_str(), 1);
                steps += 1;
            }
        }
    }

    println!("Part Two: {}", steps);

}


fn main() {
    let lines = std::fs::read_to_string("19")
        .expect("can not open file 19")
        .split('\n')
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();

    run(lines);
}
