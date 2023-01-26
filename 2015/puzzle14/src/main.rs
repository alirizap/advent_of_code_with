use std::cmp::Ordering;

#[derive(Debug, Eq)]
struct Reindeer {
    name: String,
    speed: i32,
    runtime: i32,
    resttime: i32,
    runtime_tmp: i32,
    resttime_tmp: i32,
    distance: i32,
    is_restime: bool,
    points: i32,
}

impl Reindeer {
    fn go(&mut self) {
        if !self.is_restime {
            self.distance += self.speed;
            self.runtime_tmp -= 1;
            if self.runtime_tmp == 0 {
                self.is_restime = true;
                self.runtime_tmp = self.runtime;
            }
        } else {
            self.resttime_tmp -= 1;
            if self.resttime_tmp == 0 {
                self.is_restime = false;
                self.resttime_tmp = self.resttime;
            }
        }
    }

    fn default(&mut self) {
        self.resttime_tmp = self.resttime;
        self.runtime_tmp = self.runtime;
        self.distance = 0;
        self.is_restime = false;
    }
}

impl From<String> for Reindeer {
    fn from(line: String) -> Reindeer {
        let splited_line = line.split(' ').collect::<Vec<&str>>();
        let name = splited_line[0].to_owned();
        let speed = splited_line[3].parse::<i32>().unwrap();
        let runtime = splited_line[6].parse::<i32>().unwrap();
        let resttime = splited_line[13].parse::<i32>().unwrap();
        Reindeer {
            name,
            speed,
            runtime,
            resttime,
            runtime_tmp: runtime,
            resttime_tmp: resttime,
            distance: 0,
            is_restime: false,
            points: 0,
        }
    }
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Reindeer {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

fn run(lines: Vec<String>) {
    let mut reindeers: Vec<Reindeer> = vec![];
    for line in lines {
        reindeers.push(line.into());
    }
    for _ in 0..2503 {
        for r in reindeers.iter_mut() {
            r.go();
        }
    }
    let winner = reindeers.iter().max();

    println!(
        "Part One: {} {}",
        winner.unwrap().name,
        winner.unwrap().distance
    );

    for r in reindeers.iter_mut() {
        r.default();
    }

    for _ in 0..2503 {
        for r in reindeers.iter_mut() {
            r.go();
        }
        reindeers.sort_by(|a, b| b.cmp(a));
        let top_distance = reindeers[0].distance;
        for r in reindeers.iter_mut() {
            if r.distance == top_distance {
                r.points += 1;
            }
        }
    }

    let winner = reindeers.iter().max_by_key(|r| r.points).unwrap();
    println!("Part Two: {} {}", winner.name, winner.points);
}

fn main() {
    let lines = std::fs::read_to_string("14")
        .expect("can not read file 14")
        .split('\n')
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    run(lines)
}
