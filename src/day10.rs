use crate::util;
use reduce::Reduce;
use std::collections::{HashSet, HashMap};
use petgraph::graphmap::DiGraphMap;
use petgraph::Direction::{Outgoing};
use petgraph::algo::all_simple_paths;
use regex::internal::Input;


fn connections(set: &HashSet<u32>) -> DiGraphMap<u32, ()> {
    let mut res = DiGraphMap::new();
    for &val in set {
        for i in val + 1..val + 4 {
            if set.contains(&i) {
                res.add_edge(val, i, ());
            }
        }
    }
    res
}

fn count_connections(from: u32, to: u32, gr: &DiGraphMap<u32, ()>, hist: &mut HashMap<(u32, u32), u64>) -> u64 {
    let neighbors = gr.neighbors_directed(from, Outgoing);
    if hist.contains_key(&(from, to)) {
        hist.get(&(from, to)).unwrap().clone()
    } else {
        let res = match from == to {
            true => 1,
            false => gr.neighbors_directed(from, Outgoing).map(|n| count_connections(n, to, gr, hist)).sum()
        };
        hist.insert((from, to), res);
        res
    }
}

pub fn part1(filename: &str) {
    let str = util::filename_to_string(filename).unwrap();
    let mut numbers: Vec<u32> = str.lines().map(|line| line.parse::<u32>().unwrap()).collect();
    numbers.push(0);
    numbers.sort();

    let diffs = numbers[1..].iter().zip(numbers.iter()).map(|(m, n)| m - n)
        .fold((0, 0), |acc, d|
            match d {
                1 => (acc.0 + 1, acc.1),
                3 => (acc.0, acc.1 + 1),
                _ => acc
            }
    );
    println!("{:?}", numbers);
    println!("{:?}", (diffs.1 + 1) * diffs.0)
}

pub fn part2(filename: &str) {
    let str = util::filename_to_string(filename).unwrap();
    let mut numbers: HashSet<u32> = str.lines().map(|line| line.parse::<u32>().unwrap()).collect();
    numbers.insert(0);
    let gr = connections(&numbers);
    //let res = all_simple_paths(&gr,0,*numbers.iter().max().unwrap(), 0, None);
    let mut hist: HashMap<(u32, u32), u64> = HashMap::new();
    println!("{:?}", count_connections(0, numbers.iter().max().unwrap().clone(), &gr, &mut hist));
}