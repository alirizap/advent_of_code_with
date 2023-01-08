#![allow(dead_code)]
use std::fs;
use std::collections::HashMap;


#[derive(Debug)]
struct BitwiseLogic {
    lhs: Option<String>,
    op: Option<String>,
    rhs: Option<String>,
}

impl BitwiseLogic {
    fn get_lhs(&self) -> &str {
        self.lhs.as_ref().unwrap().as_str()
    }

    fn get_op(&self) -> Option<&str> {
        self.op.as_ref().map(|value| value.as_str())
    }

    fn get_rhs(&self) -> &str {
        self.rhs.as_ref().unwrap().as_str()
    }
}

impl From<&str> for BitwiseLogic {
    fn from(value: &str) -> Self {
        let splited_vlaues: Vec<&str> = value.trim().split(' ').collect();
        let mut lhs = None;
        let mut op = None;
        let rhs;
        if splited_vlaues.len() == 3 {
            lhs = Some(splited_vlaues[0].trim().to_string());
            op = Some(splited_vlaues[1].trim().to_string());
            rhs = Some(splited_vlaues[2].trim().to_string());
        } else if splited_vlaues.len() == 2 {
            op = Some(splited_vlaues[0].trim().to_string());
            rhs = Some(splited_vlaues[1].trim().to_string());
        } else {
            rhs = Some(splited_vlaues[0].trim().to_string());
        }

        BitwiseLogic {lhs: lhs, op: op, rhs: rhs}
    }
}

fn parse_lines(line: &str) -> (&str, BitwiseLogic) {
    let values: Vec<&str> = line.split("->").collect();
    let key = values[1].trim();
    let bitwise: BitwiseLogic = values[0].into();
    (key, bitwise) 
}

fn run(lines: &Vec<&str>) {
    let mut logics = HashMap::new();
    let mut cache = HashMap::new();
    for line in lines {
        let (key, bitwise) = parse_lines(line);
        logics.insert(key, bitwise);
    }

    let result_1 = part_one(&mut cache, &logics, "a");

    println!("Part One: {}", result_1);
}

fn part_one<'a>(cache: &mut HashMap<&'a str, u16>, logics: &'a HashMap<&'a str, BitwiseLogic>, key: &'a str) -> u16 {
    if let Ok(value) = key.parse::<u16>() {
        return value;
    } else {
        match logics[key].get_op() {
            None => { 
                let v;
                if let Some(value) = cache.get(logics[key].get_rhs()) {
                    v = *value;
                } else {
                    v = part_one(cache, logics, logics[key].get_rhs());
                    cache.insert(logics[key].get_rhs(), v);
                }
                return v;
            }
            Some("AND") => {
                let v1;
                let v2;

                if let Some(value) = cache.get(logics[key].get_lhs()) {
                    v1 = *value;
                } else {
                    v1 = part_one(cache, logics, logics[key].get_lhs());
                    cache.insert(logics[key].get_lhs(), v1);
                }

                if let Some(value) = cache.get(logics[key].get_rhs()) {
                    v2 = *value;
                } else {

                    v2 = part_one(cache, logics, logics[key].get_rhs());
                    cache.insert(logics[key].get_rhs(), v2);
                }

                return  v1 & v2;
            }
            Some("OR") => {
                let v1;
                let v2;

                if let Some(value) = cache.get(logics[key].get_lhs()) {
                    v1 = *value;
                } else {
                    v1 = part_one(cache, logics, logics[key].get_lhs());
                    cache.insert(logics[key].get_lhs(), v1);
                }

                if let Some(value) = cache.get(logics[key].get_rhs()) {
                    v2 = *value;
                } else {

                    v2 = part_one(cache, logics, logics[key].get_rhs());
                    cache.insert(logics[key].get_rhs(), v2);
                }

                return  v1 | v2;
            }
            Some("LSHIFT") => {
                let v1;

                if let Some(value) = cache.get(logics[key].get_lhs()) {
                    v1 = *value;
                } else {
                    v1 = part_one(cache, logics, logics[key].get_lhs());
                    cache.insert(logics[key].get_lhs(), v1);
                }
                let v2 = logics[key].get_rhs().parse::<u16>().unwrap();
                return  v1 << v2;
            }
            Some("RSHIFT") => {
                let v1;

                if let Some(value) = cache.get(logics[key].get_lhs()) {
                    v1 = *value;
                } else {
                    v1 = part_one(cache, logics, logics[key].get_lhs());
                    cache.insert(logics[key].get_lhs(), v1);
                }
                let v2 = logics[key].get_rhs().parse::<u16>().unwrap();
                return  v1 >> v2;
            }
            Some("NOT") => {
                let v;
                if let Some(value) = cache.get(logics[key].get_rhs()) {
                    v = *value;
                } else {
                    v = part_one(cache, logics, logics[key].get_rhs());
                    cache.insert(logics[key].get_rhs(), v);
                }
                return 65535 - v;
            }
            Some(_) => unreachable!(),
        }
    }
}

fn main() {
    let contents = fs::read_to_string("7")
        .expect("can not read file 7");
    let lines: Vec<&str> = contents.split('\n')
        .map(|line| line.trim())
        .collect();

    run(&lines);
}