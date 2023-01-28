struct Ingredient(i64, i64, i64, i64, i64);

impl From<String> for Ingredient {
    fn from(value: String) -> Self {
        let splited_line = value.split(' ').collect::<Vec<&str>>();
        let capacity = splited_line[2]
            .trim_end_matches(',')
            .parse::<i64>()
            .unwrap();
        let durability = splited_line[4]
            .trim_end_matches(',')
            .parse::<i64>()
            .unwrap();
        let flavor = splited_line[6]
            .trim_end_matches(',')
            .parse::<i64>()
            .unwrap();
        let texture = splited_line[8]
            .trim_end_matches(',')
            .parse::<i64>()
            .unwrap();
        let calories = splited_line[10].parse::<i64>().unwrap();

        Self(capacity, durability, flavor, texture, calories)
    }
}

type Teaspoons = Vec<[i64; 4]>;

fn get_teaspoons() -> Teaspoons {
    let mut teaspoons: Teaspoons = vec![];
    for i in 0..101 {
        for j in 0..(101 - i) {
            for k in 0..(101 - i - j) {
                let h = 100 - i - j - k;
                teaspoons.push([i, j, k, h]);
            }
        }
    }
    teaspoons
}

fn get_score(ing: &[Ingredient], cal: i64) -> i64 {
    let teaspoons = get_teaspoons();
    let mut max = 0;

    for t in teaspoons {
        let mut capacity = ing[0].0 * t[0] + ing[1].0 * t[1] + ing[2].0 * t[2] + ing[3].0 * t[3];
        let mut durability = ing[0].1 * t[0] + ing[1].1 * t[1] + ing[2].1 * t[2] + ing[3].1 * t[3];
        let mut flavor = ing[0].2 * t[0] + ing[1].2 * t[1] + ing[2].2 * t[2] + ing[3].2 * t[3];
        let mut texture = ing[0].3 * t[0] + ing[1].3 * t[1] + ing[2].3 * t[2] + ing[3].3 * t[3];
        let calories = ing[0].4 * t[0] + ing[1].4 * t[1] + ing[2].4 * t[2] + ing[3].4 * t[3];

        if capacity < 0 {
            capacity = 0;
        }
        if durability < 0 {
            durability = 0;
        }
        if flavor < 0 {
            flavor = 0;
        }
        if texture < 0 {
            texture = 0;
        }

        if cal != 0 && calories != cal {
            continue;
        }

        let r = capacity * durability * flavor * texture;
        if r > max {
            max = r;
        }
    }
    max
}

fn run(lines: Vec<String>) {
    let mut ing: Vec<Ingredient> = vec![];
    for line in lines {
        ing.push(line.into());
    }

    let max1 = get_score(&ing, 0);
    let max2 = get_score(&ing, 500);

    println!("Part One: {}", max1);
    println!("Part Two: {}", max2);
}

fn main() {
    let lines = std::fs::read_to_string("15")
        .expect("can not read file 15")
        .split('\n')
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>();

    run(lines);
}
