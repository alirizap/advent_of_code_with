#[derive(Debug, Default)]
struct Info {
    number: String,
    children: Option<i32>,
    cats: Option<i32>,
    samoyeds: Option<i32>,
    pomeranians: Option<i32>,
    akitas: Option<i32>,
    vizslas: Option<i32>,
    goldfish: Option<i32>,
    trees: Option<i32>,
    cars: Option<i32>,
    perfumes: Option<i32>,
}

impl Info {
    fn partial_eq(&self, other: &Self, part_two: bool) -> bool {
        if self.children.is_some()
            && other.children.is_some()
            && self.children.unwrap() != other.children.unwrap()
        {
            return false;
        }

        if self.cats.is_some() && other.cats.is_some() {
            if part_two && (self.cats.unwrap() >= other.cats.unwrap()) {
                return false;
            }
            if !part_two && self.cats.unwrap() != other.cats.unwrap() {
                return false;
            }
        }

        if self.samoyeds.is_some()
            && other.samoyeds.is_some()
            && self.samoyeds.unwrap() != other.samoyeds.unwrap()
        {
            return false;
        }

        if self.pomeranians.is_some() && other.pomeranians.is_some() {
            if part_two && (self.pomeranians.unwrap() <= other.pomeranians.unwrap()) {
                return false;
            }
            if !part_two && self.pomeranians.unwrap() != other.pomeranians.unwrap() {
                return false;
            }
        }

        if self.akitas.is_some()
            && other.akitas.is_some()
            && self.akitas.unwrap() != other.akitas.unwrap()
        {
            return false;
        }

        if self.vizslas.is_some()
            && other.vizslas.is_some()
            && self.vizslas.unwrap() != other.vizslas.unwrap()
        {
            return false;
        }

        if self.goldfish.is_some() && other.goldfish.is_some() {
            if part_two && (self.goldfish.unwrap() <= other.goldfish.unwrap()) {
                return false;
            }
            if !part_two && self.goldfish.unwrap() != other.goldfish.unwrap() {
                return false;
            }
        }

        if self.trees.is_some() && other.trees.is_some() {
            if part_two && (self.trees.unwrap() >= other.trees.unwrap()) {
                return false;
            }
            if !part_two && self.trees.unwrap() != other.trees.unwrap() {
                return false;
            }
        }

        if self.cars.is_some() && other.cars.is_some() && self.cars.unwrap() != other.cars.unwrap()
        {
            return false;
        }

        if self.perfumes.is_some()
            && other.perfumes.is_some()
            && self.perfumes.unwrap() != other.perfumes.unwrap()
        {
            return false;
        }
        true
    }
}

impl From<String> for Info {
    fn from(line: String) -> Self {
        let mut info = Info::default();
        let splited_values = line.as_str().split_once(": ").unwrap();
        info.number = splited_values.0.to_string();
        let values = splited_values.1.split(',').collect::<Vec<&str>>();
        for value in values.iter() {
            let splited_value = value.split(": ").collect::<Vec<&str>>();
            let num = splited_value[1].parse::<i32>().unwrap();
            let attr = splited_value[0].trim();
            match attr {
                "children" => info.children = Some(num),
                "cats" => info.cats = Some(num),
                "samoyeds" => info.samoyeds = Some(num),
                "pomeranians" => info.pomeranians = Some(num),
                "akitas" => info.akitas = Some(num),
                "vizslas" => info.vizslas = Some(num),
                "goldfish" => info.goldfish = Some(num),
                "trees" => info.trees = Some(num),
                "cars" => info.cars = Some(num),
                "perfumes" => info.perfumes = Some(num),
                _ => unreachable!(),
            }
        }
        info
    }
}

fn run(lines: Vec<String>) {
    let mfcsm = Info {
        number: "Sue XX".to_string(),
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(2),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    let mut infos: Vec<Info> = Vec::new();
    for line in lines {
        infos.push(line.into());
    }

    for i in infos {
        if mfcsm.partial_eq(&i, false) {
            println!("Part One: {}", i.number);
        }

        if mfcsm.partial_eq(&i, true) {
            println!("Part Two: {}", i.number);
        }
    }
}

fn main() {
    let lines = std::fs::read_to_string("16")
        .expect("can not read file 16")
        .split('\n')
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();

    run(lines);
}
