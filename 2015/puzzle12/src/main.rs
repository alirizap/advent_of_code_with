use regex::Regex;
use serde_json::Value;

fn parse_json(v: &Value) -> i64 {
    match v {
        Value::Object(obj) => {
            let has_red = obj.values().find(|&value| value == "red");
            if has_red.is_some() {
                return 0;
            }
            let mut sum = 0;
            for key in obj.keys() {
                sum += parse_json(&obj[key]);
            }
            sum
        }
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(values) => {
            let mut sum = 0;
            for v in values.iter() {
                sum += parse_json(v);
            }
            sum
        }
        _ => 0,
    }
}

fn run(input: String) {
    let re = Regex::new(r"-?[0-9]+").unwrap();
    let sum: i32 = re
        .find_iter(&input)
        .filter_map(|num| num.as_str().parse().ok())
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    println!("Part One: {}", sum);
    let v: Value = serde_json::from_str(&input).unwrap();
    let sum = parse_json(&v);
    println!("Part Two: {}", sum);
}

fn main() {
    let contents = std::fs::read_to_string("12").expect("can not read file 12");
    run(contents);
}
