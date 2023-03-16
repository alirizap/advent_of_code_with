
fn next_code(prev_code: i64) -> i64 {
    (prev_code * 252533) % 33554393
}

fn next_cord(cord: (i64, i64)) -> (i64, i64) {
    let mut next_row = cord.0 - 1;
    let mut next_col = cord.1 + 1;
    if next_row < 1 {
        next_row = next_col;
        next_col = 1;
    }
    (next_row, next_col)
}

fn main() {
    let target = (2947, 3029);
    let mut cord = (1, 1);
    let mut n = 20151125;

    while cord != target {
        n = next_code(n);
        cord = next_cord(cord);
    }
    println!("Answer: {}", n);
}

