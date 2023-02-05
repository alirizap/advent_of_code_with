fn factors(num: u32) -> Vec<u32> {
    let mut f = Vec::new();
    let rng = f32::sqrt(num as f32) as u32;
    for i in 1..rng + 1 {
        if num % i == 0 {
            f.push(i);
            f.push(num / i);
        }
    }
    f
}

fn run(input: u32) {
    let mut door = 1;
    loop {
        let factors = factors(door);
        let sum: u32 = factors.iter().sum();
        if sum * 10 >= input {
            break;
        }
        door += 1;
    }
    println!("{}", door);
}

fn main() {
    let input = std::env::args()
        .nth(1)
        .expect("cannot get input from stdin")
        .parse::<u32>()
        .expect("invalid input");

    run(input);
}
