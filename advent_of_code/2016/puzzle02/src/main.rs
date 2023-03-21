type Keypad = Vec<Vec<Option<char>>>;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(s: char) -> Self {
        match s {
            'U' => Self::Up,
            'D' => Self::Down,
            'R' => Self::Right,
            'L' => Self::Left,
            _ => panic!("invalid direction: {}", s),
        }
    }
}

struct State {
    row: usize,
    col: usize,
    lower_bound: usize,
    upper_bound: usize,
    keypad: Keypad,
}

impl State {
    fn new(row: usize, col: usize, lower_bound: usize, upper_bound: usize, keypad: Keypad) -> Self {
        State {
            row,
            col,
            lower_bound,
            upper_bound,
            keypad,
        }
    }

    fn get_key(&self) -> Option<char> {
        self.keypad[self.row][self.col]
    }

    fn walk(&mut self, dir: Direction) {
        match dir {
            Direction::Up
                if (self.row != self.lower_bound)
                    && (self.keypad[self.row - 1][self.col].is_some()) =>
            {
                self.row -= 1
            }
            Direction::Down
                if (self.row != self.upper_bound)
                    && (self.keypad[self.row + 1][self.col].is_some()) =>
            {
                self.row += 1
            }
            Direction::Left
                if (self.col != self.lower_bound)
                    && (self.keypad[self.row][self.col - 1].is_some()) =>
            {
                self.col -= 1
            }
            Direction::Right
                if (self.col != self.upper_bound)
                    && (self.keypad[self.row][self.col + 1].is_some()) =>
            {
                self.col += 1
            }
            _ => (),
        }
    }
}


fn solve(instruct: &str, mut state: State) -> String {
    let mut code = String::new();

    for line in instruct.lines() {
        for dir in line.chars() {
            state.walk(dir.into());
        }
        code.push(state.get_key().unwrap())
    }

   code
}

fn main() {
    let instruct = std::fs::read_to_string("2").expect("can not read file 2");

    // Part One
    let keypad: Keypad = vec![
        vec![Some('1'), Some('2'), Some('3')],
        vec![Some('4'), Some('5'), Some('6')],
        vec![Some('7'), Some('8'), Some('9')],
    ];
    let state = State::new(1, 1, 0, 2, keypad);
    let mut code = solve(&instruct, state);
    println!("Part One: {}", code);

    // Part Two
    let keypad: Keypad = vec![
        vec![None, None, Some('1'), None, None],
        vec![None, Some('2'), Some('3'), Some('4'), None],
        vec![Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
        vec![None, Some('A'), Some('B'), Some('C'), None],
        vec![None, None, Some('D'), None, None],
    ];

    let state = State::new(2, 0, 0, 4, keypad);
    code.clear();
    code = solve(&instruct, state);

    println!("Part Two: {}", code);
    
}
