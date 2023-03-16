use std::fs;


struct Command {
    kind: String, 
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let kind;
        let start_row;
        let start_col;
        let end_row;
        let end_col;
        let line_split: Vec<&str> = value.split(' ').collect();
        let len = line_split.len();
        let start_range_index;

        if len == 5 {
            start_range_index = 2;
            kind = if line_split[1] == "on" {
                "turn on".to_string()
            } else {
                "turn off".to_string()
            };
        } else {
            start_range_index = 1;
            kind = "toggle".to_string();
        }

        let mut start_range = line_split[start_range_index]
                            .split(",")
                            .map(|num| num.parse::<usize>().unwrap());
        start_row = start_range.next().unwrap();
        start_col = start_range.next().unwrap();

        let mut end_range = line_split[len-1]
                            .split(",")
                            .map(|num| num.parse::<usize>().unwrap());
        end_row = end_range.next().unwrap() + 1;
        end_col = end_range.next().unwrap() + 1;

        Command {
            kind,
            start_row,
            start_col,
            end_row,
            end_col
        }
    }
}

fn run(lines: Vec<&str>) {
    let mut grid_1 = vec![vec![0; 1000]; 1000];
    let mut grid_2 = vec![vec![0; 1000]; 1000];
    for line in lines {
        let cmd: Command = line.into();
        for i in cmd.start_row..cmd.end_row {
            for j in cmd.start_col..cmd.end_col {
                match cmd.kind.as_str() {
                    "turn on" => { grid_1[i][j] = 1; grid_2[i][j] = grid_2[i][j] + 1; }
                    "turn off" => { grid_1[i][j] = 0; grid_2[i][j] = usize::saturating_sub(grid_2[i][j], 1); }
                    "toggle" => { grid_1[i][j] = 1 - grid_1[i][j]; grid_2[i][j] = grid_2[i][j] + 2; }
                    _ => unreachable!(),
                }
            }
        }
    }

    let mut count_1 = 0;
    let mut brightness = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if grid_1[i][j] == 1 {
                count_1 += 1;
            }
            brightness += grid_2[i][j];
        }
    }

    println!("Part One: {}", count_1);
    println!("Part Two: {}", brightness);
}


fn main() {
    let contents = fs::read_to_string("6").expect("can not read file 6");
    let lines: Vec<&str> = contents.split('\n')
                        .map(|line| line.trim())
                        .collect();
    run(lines);
}