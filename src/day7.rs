use crate::util;

use regex::Regex;
use std::collections::{HashSet, BTreeMap};
use petgraph::graphmap::{DiGraphMap, NeighborsDirected};
use std::iter;
use petgraph::Direction::{Outgoing, Incoming};
use petgraph::{Directed, Graph};
use reduce::Reduce;

#[derive(Debug)]
struct Bag {
    container: String,
    containes: Vec<(String, u32)>,
}

lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(r"(?P<color>\w+ \w+) bags contain (?P<content>.*?).\n").unwrap();
    static ref BAGGING: regex::Regex = regex::Regex::new(r"(?P<number>\d+) (?P<bags>\w+ \w+)s?,? ?").unwrap();
        }


fn input_to_tree(inp: String) -> BTreeMap<String, Vec<(String, u32)>>{
    let mut bags: BTreeMap<String, (Vec<(String, u32)>)> = BTreeMap::new();
    for capture in RE.captures_iter(inp.as_str()) {
        let mut subbags = vec![];
        for subcap in BAGGING.captures_iter(capture.name("content").unwrap().as_str()) {
            subbags.push((subcap["bags"].to_string(), subcap["number"].parse::<u32>().unwrap()));
        };
        bags.insert(capture["color"].to_string(), subbags);
    }
    bags
}

fn tree_to_graph(tree: &BTreeMap<String, Vec<(String, u32)>>) -> DiGraphMap<&str, u32> {
    let edges = tree.into_iter()
        .flat_map(|(k, vs)| iter::repeat(k).zip(vs))
        .map(|a| (a.0.as_str(), a.1.0.as_str(), a.1.1));
    edges.collect()
}

fn walk_up<'a>(bag: &'a str, gr: &DiGraphMap<&'a str, u32>) -> HashSet<&'a str> {
    let up: HashSet<& str> = gr.neighbors_directed(bag, Incoming).collect();
    let res =up.iter().map(|n| walk_up(n, &gr))
        .reduce(|a, b| a.union(&b).copied().collect());
    match res {
        Some(a) => a.union(&up).copied().collect(),
        None => up
    }
}

fn walk_down<'a>(bag: &'a str, gr: &DiGraphMap<&'a str, u32>) -> u32 {
    let down: HashSet<& str> = gr.neighbors_directed(bag, Outgoing).collect();
    let res =down.iter().map(|n| gr.edge_weight(bag, n).unwrap()*(1+walk_down(n, &gr)))
        .reduce(|a, b| a+b);
    match res {
        Some(a) => a,
        None => 0
    }
}

pub fn part1(filename: &str) {
    let str = util::filename_to_string(filename).unwrap();
    let bags = input_to_tree(str);
    let gr = tree_to_graph(&bags);
    let res = walk_up("shiny gold", &gr);
    println!("{}, {:?}",res.len(), res );
}

pub fn part2(filename: &str) {
    let str = util::filename_to_string(filename).unwrap();
    let bags = input_to_tree(str);
    let gr = tree_to_graph(&bags);
    let res = walk_down("shiny gold", &gr);
    println!("{}", res );
}
