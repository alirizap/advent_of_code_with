// very slow solution
use std::collections::{HashMap, HashSet};

fn subset_sum(
    sets: &mut HashSet<Vec<usize>>,
    weights: &Vec<usize>,
    target: usize,
    partial: Vec<usize>,
) {
    let partial_sum: usize = partial.iter().sum();
    if partial_sum == target {
        sets.insert(partial.clone());
    }

    if partial_sum > target {
        return;
    }

    for i in 0..weights.len() {
        let n = weights[i];
        let mut next_partial = partial.clone();
        next_partial.push(n);

        let mut remaining: Vec<usize> = Vec::new();
        for j in i + 1..weights.len() {
            remaining.push(weights[j]);
        }
        subset_sum(sets, &remaining, target, next_partial)
    }
}

fn find_quantum_entanglement(weights: &Vec<usize>, packs: usize) -> usize {
    let sum: usize = weights.iter().sum();
    let package_balance_weight = sum / packs;
    let mut groups: HashSet<Vec<usize>> = HashSet::new();
    subset_sum(
        &mut groups,
        &weights,
        package_balance_weight,
        Vec::<usize>::new(),
    );

    let mut group_sizes: HashMap<usize, HashSet<Vec<usize>>> = HashMap::new();
    for packages in groups.iter() {
        let packages_count = packages.len();
        group_sizes
            .entry(packages_count)
            .and_modify(|group| {
                group.insert(packages.clone());
            })
            .or_insert({
                let mut hs = HashSet::new();
                hs.insert(packages.clone());
                hs
            });
    }

    let mut smallest = usize::MAX;
    for (size, _) in group_sizes.iter() {
        if *size < smallest {
            smallest = *size
        }
    }

    let mut minimum = usize::MAX;
    for packages in group_sizes.get(&smallest) {
        for p in packages.iter() {
            let product = p.iter().copied().reduce(|a, b| a*b).unwrap();
            if product < minimum {
                minimum = product;
            }
        }
    }

    minimum
}

fn main() {
    let weights = std::fs::read_to_string("24")
        .expect("can not read file 24")
        .split('\n')
        .map(|number| number.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let part1 = find_quantum_entanglement(&weights, 3);
    println!("Part one: {}", part1);
    let part2 = find_quantum_entanglement(&weights, 4);
    println!("Part two: {}", part2);
}
