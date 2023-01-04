use std::fs;

#[derive(Debug)]
struct Dimensions {
    l: i32,
    w: i32,
    h: i32
}

impl Dimensions {
    fn surface_area(&self) -> i32 {
        let surface = 2*self.l*self.w + 2*self.w*self.h + 2*self.h*self.l;
        let mut min_area = std::cmp::min(self.l*self.w, self.w*self.h);
        min_area = std::cmp::min(min_area, self.h*self.l);
        surface + min_area
    }

    fn ribbon_length(&self) -> i32 {
        let mut values = self.to_array();
        values.sort();
        (values[0] * 2 + values[1] * 2) + (self.l * self.w * self.h)
    }

    fn to_array(&self) -> [i32; 3] {
        [self.l, self.w, self.h]
    }
}

impl<'a> From<&'a str> for Dimensions {
    fn from(value: &'a str) -> Self {
        let ds: Vec<&str> = value.trim().split('x').collect();
        Self {
            l: ds[0].parse().unwrap(),
            w: ds[1].parse().unwrap(),
            h: ds[2].parse().unwrap()
        }
    }
}

fn run(input: &str) {
    let contents = fs::read_to_string(input)
        .expect("unable to read input file");

    let lines: Vec<&str> = contents
            .trim().split('\n').collect();
    let mut dims: Vec<Dimensions> = Vec::with_capacity(1000);
    for line in lines.into_iter() {
        dims.push(line.into())
    }
    let mut sum = 0;
    let mut ribbon_length = 0;
    for dim in dims.iter() {
        sum += dim.surface_area();
        ribbon_length += dim.ribbon_length();
    }
    println!("Part One: {}", sum);
    println!("Part Two: {}", ribbon_length);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).expect("filepath not provided");
    run(&input);
}