use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Edge {
    person: String,
    happiness: i32,
}

impl Edge {
    fn new(person: String, happiness: i32) -> Self {
        Self { person, happiness }
    }
}

type Graph = HashMap<String, Vec<Edge>>;

fn make_graph(lines: Vec<String>) -> Graph {
    let mut graph: Graph = HashMap::new();
    for line in lines {
        let splited_line: Vec<&str> = line.split(' ').collect();
        let p1 = splited_line[0].to_string();
        let mut p2 = splited_line[10].to_string();
        p2.pop().unwrap();
        let mut happiness = splited_line[3].parse::<i32>().unwrap();
        if splited_line[2] == "lose" {
            happiness = -happiness;
        }
        match graph.get_mut(&p2) {
            None => (),
            Some(ref mut edges) => {
                for e in edges.iter_mut() {
                    if e.person == p1 {
                        happiness += e.happiness;
                        e.happiness = happiness;
                    }
                }
            }
        }
        graph
            .entry(p1)
            .and_modify(|v| v.push(Edge::new(p2.clone(), happiness)))
            .or_insert_with(|| vec![Edge::new(p2.clone(), happiness)]);
    }
    graph
}

fn happiness_calc(graph: &Graph) -> i32 {
    let keys = graph.keys().map(|k| k.to_string()).collect::<Vec<String>>();
    let mut len = keys.len();
    let person_perms = keys.into_iter().permutations(len);
    let mut max_happiness = 0;
    for persons in person_perms {
        let mut i = 0;
        let mut j = 1;
        let mut h = 0;
        len = persons.len();
        while j != len {
            for edge in &graph[&persons[i]] {
                if edge.person == persons[j] {
                    h += edge.happiness;
                    break;
                }
            }
            i += 1;
            j += 1;
        }
        let last_person = persons[len - 1].as_str();
        for edge in &graph[last_person] {
            if edge.person == persons[0] {
                h += edge.happiness;
                break;
            }
        }
        if h > max_happiness {
            max_happiness = h;
        }
    }
    max_happiness
}

fn run(lines: Vec<String>) {
    let mut graph = make_graph(lines);
    let part_one = happiness_calc(&graph);
    println!("Part One: {}", part_one);

    for (_, edges) in graph.iter_mut() {
        edges.push(Edge::new("me".to_string(), 0));
    }
    graph.insert("me".to_string(), vec![]);
    let keys = graph.keys().map(|k| k.to_string()).collect::<Vec<String>>();
    for k in keys {
        graph.get_mut("me").map(|edges| edges.push(Edge::new(k, 0)));
    }
    let part_two = happiness_calc(&graph);
    println!("Part Two: {}", part_two);
}

fn main() {
    let lines = std::fs::read_to_string("13")
        .expect("can not read file 13")
        .split('\n')
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();

    run(lines);
}
