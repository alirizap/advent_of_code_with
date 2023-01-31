use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    On,
    Off,
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(i32, i32);

type Grid = HashMap<Point, State>;

fn parse_input(input: &[String]) -> Grid {
    let mut grid = Grid::new();
    for (x, line) in input.iter().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            match ch {
                '.' => grid.insert(
                    Point(x.try_into().unwrap(), y.try_into().unwrap()),
                    State::Off,
                ),
                '#' => grid.insert(
                    Point(x.try_into().unwrap(), y.try_into().unwrap()),
                    State::On,
                ),
                _ => None,
            };
        }
    }
    grid
}

fn find_neighbors(p: Point) -> Vec<Point> {
    vec![
        Point(p.0 - 1, p.1),
        Point(p.0 + 1, p.1),
        Point(p.0, p.1 - 1),
        Point(p.0, p.1 + 1),
        Point(p.0 - 1, p.1 - 1),
        Point(p.0 - 1, p.1 + 1),
        Point(p.0 + 1, p.1 - 1),
        Point(p.0 + 1, p.1 + 1),
    ]
}

fn lights_on(lines: &[String], part_two: bool) -> i32 {
    let mut grid = parse_input(lines);
    let mut i = 0;
    if part_two {
        grid.insert(Point(0, 0), State::On);
        grid.insert(Point(0, 99), State::On);
        grid.insert(Point(99, 0), State::On);
        grid.insert(Point(99, 99), State::On);
    }
    while i < 100 {
        let mut new_grid = Grid::new();
        for (point, state) in grid.iter() {
            let neighbors = find_neighbors(*point);
            let mut count_on = 0;
            for p in neighbors {
                if let Some(State::On) = grid.get(&p) {
                    count_on += 1;
                }
            }

            let new_point = Point(point.0, point.1);
            match state {
                State::On if count_on == 2 || count_on == 3 => {
                    new_grid.insert(new_point, State::On)
                }
                State::On => new_grid.insert(new_point, State::Off),
                State::Off if count_on == 3 => new_grid.insert(new_point, State::On),
                State::Off => new_grid.insert(new_point, State::Off),
            };
        }

        if part_two {
            new_grid.insert(Point(0, 0), State::On);
            new_grid.insert(Point(0, 99), State::On);
            new_grid.insert(Point(99, 0), State::On);
            new_grid.insert(Point(99, 99), State::On);
        }
        i += 1;
        grid = new_grid;
    }

    let mut count_on = 0;
    for state in grid.values() {
        if *state == State::On {
            count_on += 1;
        }
    }

    count_on
}

fn run(lines: Vec<String>) {
    println!("Part One: {}", lights_on(&lines, false));
    println!("Part Two: {}", lights_on(&lines, true));
}

fn main() {
    let lines = std::fs::read_to_string("18")
        .expect("can not read file 18")
        .split('\n')
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>();

    run(lines);
}
