#[derive(Copy, Clone)]
enum Dir {
    N,
    S,
    W,
    E,
}

#[derive(Copy, Clone)]
struct Pos {
    face: Dir,
    x: i32,
    y: i32,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Pos {
    fn new() -> Self {
        Self {
            face: Dir::N,
            x: 0,
            y: 0,
        }
    }

    fn change_dir(&mut self, d: &str) {
        self.face = match self.face {
            Dir::N => {
                if d.starts_with('L') {
                    Dir::W
                } else {
                    Dir::E
                }
            }
            Dir::E => {
                if d.starts_with('L') {
                    Dir::N
                } else {
                    Dir::S
                }
            }
            Dir::S => {
                if d.starts_with('L') {
                    Dir::E
                } else {
                    Dir::W
                }
            }
            Dir::W => {
                if d.starts_with('L') {
                    Dir::S
                } else {
                    Dir::N
                }
            }
        }
    }

    fn walk(&mut self) {
        match self.face {
            Dir::N => self.y += 1,
            Dir::S => self.y -= 1,
            Dir::W => self.x -= 1,
            Dir::E => self.x += 1,
        }
    }

    fn position(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn run(directions: Vec<&str>) {
    let mut curr_pos = Pos::new();

    let mut visited: Vec<Pos> = Vec::new();
    let mut twice_visited = Pos::new();
    let mut found = false;

    for d in directions.iter() {
        let step = d[1..].parse::<i32>().unwrap();
        curr_pos.change_dir(d);
        for _ in 1..step + 1 {
            curr_pos.walk();
            if !found {
                match visited.iter().find(|x| **x == curr_pos) {
                    None => visited.push(curr_pos),
                    Some(pos) => {
                        twice_visited = *pos;
                        found = true;
                    }
                }
            }
        }
    }

    println!("Part One: {}", curr_pos.position());
    println!("Part Two: {}", twice_visited.position());
}

fn main() {
    let data = std::fs::read_to_string("1").expect("can not read file 1");
    let directions = data.split(',').map(|x| x.trim()).collect::<Vec<&str>>();
    run(directions);
}
