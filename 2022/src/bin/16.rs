use core::time;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
};

use petgraph::{
    algo::{all_simple_paths, dijkstra, min_spanning_tree},
    data::FromElements,
    dot::{Config, Dot},
    prelude::*,
};

fn main() {
    println!("01: {}", part01(include_str!("../inputs/16")));
    println!("02: {}", part02(include_str!("../inputs/16")));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    label: String,
    flow_rate: i32,
}

impl Valve {
    fn new(label: &str, flow_rate: i32) -> Self {
        Self {
            label: label.to_string(),
            flow_rate,
        }
    }
}

impl Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.label, self.flow_rate)
    }
}

fn parse(input: &str) -> (HashSet<Valve>, HashSet<(String, String)>) {
    let mut valves = HashSet::new();
    let mut tunnels = HashSet::new();
    let re_valve = regex::Regex::new(
        r"^Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? ((:?\w\w(:?, )?)*)$",
    )
    .unwrap();
    for line in input.lines() {
        let caps = re_valve.captures(line).unwrap();
        let valve = Valve::new(
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str().parse().unwrap(),
        );
        let connected = caps.get(3).unwrap().as_str().split(", ");
        for label in connected {
            tunnels.insert((valve.label.clone(), label.to_string()));
        }
        valves.insert(valve);
    }
    (valves, tunnels)
}

fn build_graph(
    (valves, tunnels): (HashSet<Valve>, HashSet<(String, String)>),
) -> UnGraph<Valve, u32> {
    let mut graph = UnGraph::<Valve, u32>::new_undirected();
    let mut node_map = HashMap::new();
    for valve in &valves {
        let node_index = graph.add_node(valve.clone());
        node_map.insert(valve.label.clone(), node_index);
    }
    for tunnel in &tunnels {
        let from = node_map.get(&tunnel.0).unwrap();
        let to = node_map.get(&tunnel.1).unwrap();
        if !graph.contains_edge(*from, *to) {
            graph.add_edge(*from, *to, 1);
        }
    }
    graph
}

fn part01(input: &str) -> i32 {
    let (valves, tunnels) = parse(input);
    let mut graph = UnGraph::<Valve, i32>::new_undirected();
    let mut node_map = HashMap::new();
    for valve in &valves {
        let node_index = graph.add_node(valve.clone());
        node_map.insert(valve.label.clone(), node_index);
    }
    for tunnel in &tunnels {
        let from = node_map.get(&tunnel.0).unwrap();
        let to = node_map.get(&tunnel.1).unwrap();
        if !graph.contains_edge(*from, *to) {
            graph.add_edge(*from, *to, 1);
        }
    }

    println!("{}", Dot::new(&graph));

    // simplify graph
    while let Some(ni) = graph
        .node_indices()
        .find(|ni| graph[*ni].flow_rate == 0 && graph.neighbors_undirected(*ni).count() <= 2)
    {
        let edges = graph.neighbors(ni).collect::<Vec<_>>();
        if edges.len() == 2 {
            let from = edges[0];
            let to = edges[1];
            let from_weight = *graph.edges_connecting(ni, from).next().unwrap().weight();
            let to_weight = *graph.edges_connecting(ni, to).next().unwrap().weight();
            graph.add_edge(from, to, from_weight + to_weight);
        }

        graph.remove_node(ni);
    }

    println!("{}", Dot::new(&graph));

    todo!()
}

fn calc_potentials(
    graph: &UnGraph<Valve, i32>,
    time_left: i32,
    opened: HashSet<NodeIndex>,
    from: NodeIndex,
) -> Vec<(NodeIndex, i32)> {
    dijkstra(&graph, from, None, |er| *er.weight())
        .iter()
        .map(|(ni, cost)| {
            // when opened all potential is lost since potential is all pressure released
            // for the remaining time, hence we can consider flow rate to be zero
            let flow_rate = if opened.contains(ni) {
                0
            } else {
                graph[*ni].flow_rate
            };
            (*ni, flow_rate * (time_left - cost - 1))
        })
        .collect()
}

fn bfs(graph: &UnGraph<Valve, i32>, start: NodeIndex) {
    let mut frontier = VecDeque::new();
    frontier.push_back(start);
    let mut opened = HashSet::new();
    opened.insert(start);

    while let Some(current) = frontier.pop_front() {
        let candidates = dijkstra(&graph, current, None, |er| *er.weight())
            .iter()
            .map(|(ni, cost)| (*ni, *cost))
            .filter(|(ni, _)| !opened.contains(ni))
            .collect::<Vec<_>>();
        for (next, cost) in candidates {
            frontier.push_back(next);
        }
    }
}

fn part02(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (valves, tunnels) = parse(EXAMPLE);
        assert!(valves.contains(&Valve::new("AA", 0)));
        assert!(valves.contains(&Valve::new("BB", 13)));
        assert!(valves.contains(&Valve::new("CC", 2)));
        assert!(valves.contains(&Valve::new("DD", 20)));
        assert!(valves.contains(&Valve::new("EE", 3)));
        assert!(valves.contains(&Valve::new("FF", 0)));
        assert!(valves.contains(&Valve::new("GG", 0)));
        assert!(valves.contains(&Valve::new("HH", 22)));
        assert!(valves.contains(&Valve::new("II", 0)));
        assert!(valves.contains(&Valve::new("JJ", 21)));
        assert_eq!(valves.len(), 10);
        assert!(tunnels.contains(&("AA".to_string(), "DD".to_string())));
        assert!(tunnels.contains(&("AA".to_string(), "II".to_string())));
        assert!(tunnels.contains(&("AA".to_string(), "BB".to_string())));
        assert!(tunnels.contains(&("BB".to_string(), "CC".to_string())));
        assert!(tunnels.contains(&("BB".to_string(), "AA".to_string())));
        assert!(tunnels.contains(&("CC".to_string(), "DD".to_string())));
        assert!(tunnels.contains(&("CC".to_string(), "BB".to_string())));
        assert!(tunnels.contains(&("DD".to_string(), "CC".to_string())));
        assert!(tunnels.contains(&("DD".to_string(), "AA".to_string())));
        assert!(tunnels.contains(&("DD".to_string(), "EE".to_string())));
        assert!(tunnels.contains(&("EE".to_string(), "FF".to_string())));
        assert!(tunnels.contains(&("EE".to_string(), "DD".to_string())));
        assert!(tunnels.contains(&("FF".to_string(), "EE".to_string())));
        assert!(tunnels.contains(&("FF".to_string(), "GG".to_string())));
        assert!(tunnels.contains(&("GG".to_string(), "FF".to_string())));
        assert!(tunnels.contains(&("GG".to_string(), "HH".to_string())));
        assert!(tunnels.contains(&("HH".to_string(), "GG".to_string())));
        assert!(tunnels.contains(&("II".to_string(), "AA".to_string())));
        assert!(tunnels.contains(&("II".to_string(), "JJ".to_string())));
        assert!(tunnels.contains(&("JJ".to_string(), "II".to_string())));
        assert_eq!(tunnels.len(), 20);
    }

    #[test]
    fn example01() {
        assert_eq!(part01(EXAMPLE), -1);
    }

    #[test]
    fn example02() {
        assert_eq!(part02(EXAMPLE), -1);
    }

    const EXAMPLE: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;
}
