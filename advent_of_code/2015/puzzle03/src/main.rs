use parts::{part1, part2};
use std::fs;


fn main() {
    let contents = fs::read_to_string("3")
        .expect("can not read file 3");

    part1::run(&contents);
    part2::run(&contents);
}