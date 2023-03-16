use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug)]
struct Edge {
    cost: u32,
    dest: String,
}

fn make_graph(paths: &Vec<&str>) -> HashMap<String, Vec<Edge>> {
    let mut graph: HashMap<String, Vec<Edge>> = HashMap::new();
    for path in paths {
        let path_splited: Vec<&str> = path.split(' ').collect();
        let source = String::from(path_splited[0]);
        let dest = String::from(path_splited[2]);
        let cost = path_splited[4].parse::<u32>().unwrap();

        graph
            .entry(source.clone())
            .and_modify(|e| {
                e.push(Edge {
                    cost,
                    dest: dest.clone(),
                })
            })
            .or_insert_with(|| {
                vec![Edge {
                    cost,
                    dest: dest.clone(),
                }]
            });

        graph
            .entry(dest)
            .and_modify(|e| {
                e.push(Edge {
                    cost,
                    dest: source.clone(),
                })
            })
            .or_insert_with(|| vec![Edge { cost, dest: source }]);
    }
    graph
}

fn get_cities(paths: &Vec<&str>) -> HashSet<String> {
    let mut cities = HashSet::new();
    for path in paths {
        let path_splited: Vec<&str> = path.split(' ').collect();
        let source = String::from(path_splited[0]);
        let dest = String::from(path_splited[2]);

        cities.insert(source);
        cities.insert(dest);
    }
    cities
}

fn run(lines: Vec<&str>) {
    let graph = make_graph(&lines);
    let cities = get_cities(&lines);
    let len = cities.len();
    let all_paths = cities.into_iter().permutations(len);
    let mut costs = BTreeSet::new();

    for path in all_paths {
        let mut current_city = &path[0];
        let mut next_city_idx = 1;

        let mut cost = 0;
        while next_city_idx < path.len() {
            let next_city = &path[next_city_idx];
            let edge = graph[current_city]
                .iter()
                .find(|&edge| &edge.dest == next_city);
            match edge {
                Some(e) => cost += e.cost,
                None => break,
            }

            current_city = next_city;
            next_city_idx += 1;
        }

        costs.insert(cost);
    }
    let min_cost = costs.iter().next();
    let max_cost = costs.iter().next_back();

    println!("Part One: {}", min_cost.unwrap());
    println!("Part Two: {}", max_cost.unwrap());
}

fn main() {
    let contents = std::fs::read_to_string("9").expect("can not read file 9");

    let lines: Vec<&str> = contents.split('\n').map(|line| line.trim()).collect();

    run(lines);
}
