enum Dir {
    N,
    S,
    W,
    E,
}

struct Pos(i32, i32);

fn run(directions: Vec<&str>) {
    let mut curr_dir = Dir::N;
    let mut curr_pos = Pos(0, 0);

    for d in directions.iter() {
        let step = d[1..].parse::<i32>().unwrap();
        if d.starts_with('L') {
            match curr_dir {
                Dir::N => { curr_dir = Dir::W; curr_pos.0 = curr_pos.0 - step; }
                Dir::E => { curr_dir = Dir::N; curr_pos.1 = curr_pos.1 + step; }
                Dir::S => { curr_dir = Dir::E; curr_pos.0 = curr_pos.0 + step; }
                Dir::W => { curr_dir = Dir::S; curr_pos.1 = curr_pos.1 - step; }
            }
        } else if d.starts_with('R') {
            match curr_dir {
                Dir::N => { curr_dir = Dir::E; curr_pos.0 = curr_pos.0 + step; }
                Dir::E => { curr_dir = Dir::S; curr_pos.1 = curr_pos.1 - step; }
                Dir::S => { curr_dir = Dir::W; curr_pos.0 = curr_pos.0 - step; }
                Dir::W => { curr_dir = Dir::N; curr_pos.1 = curr_pos.1 + step; }
            }
        }
    }

    println!("Part One: {}", curr_pos.0.abs() + curr_pos.1.abs());
}

fn main() {
    let data = std::fs::read_to_string("1").expect("can not read file 1");
    let directions = data.split(',').map(|x| x.trim()).collect::<Vec<&str>>();
    run(directions);
}
