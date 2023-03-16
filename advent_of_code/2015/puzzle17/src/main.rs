use std::ops::AddAssign;
use std::iter::{Iterator, Sum};

#[derive(Debug, Clone, Copy, Default)]
struct Container(i32);

type Combination = Vec<Container>;

impl AddAssign for Container {
    fn add_assign(&mut self, other: Self) {
        *self = Container(self.0 + other.0);
    }
}

impl Sum for Container {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        let mut container = Container(0);
        for c in iter {
            container += c;
        }
        container
    }
}

impl PartialEq for Container {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl From<String> for Container {
    fn from(value: String) -> Self {
        Container(value.parse::<i32>().unwrap())
    }
}

fn combination(
    arr: &Vec<Container>,
    n: usize,
    r: usize,
    index: usize,
    data: &mut Vec<Container>,
    i: usize,
    combs: &mut Vec<Combination>,
) {
    if index == r {
        let mut tmp = vec![];
        for item in data.iter().take(r) {
            tmp.push(*item);
        }
        combs.push(tmp);
        return;
    }

    if i >= n {
        return;
    }

    data[index] = arr[i];

    combination(arr, n, r, index + 1, data, i + 1, combs);
    combination(arr, n, r, index, data, i + 1, combs);
}

fn run(lines: Vec<String>) {
    let mut arr = vec![];
    for line in lines {
        arr.push(line.into());
    }
    let mut data: Vec<Container> = vec![Container::default(); arr.len()];
    let target = Container(150);
    let mut count = 0;
    let mut minmum_containers = arr.len();
    let mut count_min = 0;

    for i in 1..arr.len() {
        let mut combs: Vec<Combination> = vec![];
        combination(&arr, arr.len(), i, 0, &mut data, 0, &mut combs);
        for comb in combs.into_iter() {
            let sum: Container = comb.into_iter().sum();
            if sum == target {
                count += 1;
                if i <= minmum_containers {
                    minmum_containers = i;
                    count_min += 1;
                }
            }
        }
    }
    println!("Part One: {}", count);
    println!("Part Two: {}", count_min);
}

fn main() {
    let lines = std::fs::read_to_string("17")
        .expect("can not read file 17")
        .split('\n')
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>();

    run(lines);
}
