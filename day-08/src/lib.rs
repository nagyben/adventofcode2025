type Point3D = (usize, usize, usize);
use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    multi::separated_list1,
};
use petgraph::{algo::connected_components, prelude::UnGraphMap, visit::Bfs};

pub fn part1(input: &str, n: usize) -> String {
    let nodes = parse_input(input);

    // create a fully connected graph with distances as weights
    let mut node_distances = nodes
        .clone()
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let dist = euler_distance(a, b) as isize;
            (a, b, dist)
        })
        .filter(|(_, _, dist)| *dist > 0) // remove self-loops
        .dedup_by(|a, b| (a.0 == b.0 && a.1 == b.1) || (a.1 == b.0 && a.0 == b.1)) // remove duplicate edges
        .collect::<Vec<(Point3D, Point3D, isize)>>();

    node_distances.sort_by(|(_, _, dist_a), (_, _, dist_b)| dist_a.cmp(dist_b));

    let connections = &node_distances[0..n];
    let mut graph: UnGraphMap<Point3D, usize> = UnGraphMap::new();

    for node in nodes.iter() {
        graph.add_node(*node);
    }
    for (a, b, dist) in connections {
        graph.add_edge(*a, *b, *dist as usize);
    }

    let mut visited = HashSet::new();
    let mut circuits = Vec::new();
    for node in graph.nodes() {
        let mut num_connections = 0;
        let mut bfs = Bfs::new(&graph, node);
        while let Some(nx) = bfs.next(&graph) {
            if visited.contains(&nx) {
                break;
            }
            visited.insert(nx);
            num_connections += 1;
        }
        circuits.push(num_connections);
    }
    println!("{:?}", circuits);
    circuits.sort();
    circuits.reverse();
    circuits.iter().take(3).product::<usize>().to_string()
}

pub fn part2(input: &str) -> String {
    let nodes = parse_input(input);

    // create a fully connected graph with distances as weights
    let mut node_distances = nodes
        .clone()
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let dist = euler_distance(a, b) as isize;
            (a, b, dist)
        })
        .filter(|(_, _, dist)| *dist > 0) // remove self-loops
        .dedup_by(|a, b| (a.0 == b.0 && a.1 == b.1) || (a.1 == b.0 && a.0 == b.1)) // remove duplicate edges
        .collect::<Vec<(Point3D, Point3D, isize)>>();

    node_distances.sort_by(|(_, _, dist_a), (_, _, dist_b)| dist_a.cmp(dist_b));

    let mut graph: UnGraphMap<Point3D, usize> = UnGraphMap::new();

    for node in nodes.iter() {
        graph.add_node(*node);
    }

    // pop() only works from the end so we have to reverse the list
    node_distances.reverse();
    let mut last_edge: Option<(Point3D, Point3D, isize)> = None;
    while connected_components(&graph) > 1 {
        let (a, b, dist) = node_distances
            .pop()
            .expect("Ran out of edges before graph was fully connected");
        graph.add_edge(a, b, dist as usize);
        last_edge = Some((a, b, dist));
    }
    let last_edge = last_edge.expect("No edges were added to the graph");
    println!("last_edge: {:?}", last_edge);
    (last_edge.0.0 * last_edge.1.0).to_string()
}

fn parse_input(input: &str) -> Vec<(usize, usize, usize)> {
    separated_list1(
        newline::<&str, nom::error::Error<&str>>,
        separated_list1(tag(","), digit1.map(|d: &str| d.parse::<usize>().unwrap())),
    )
    .parse(input)
    .unwrap()
    .1
    .iter()
    .map(|v: &Vec<usize>| (v[0], v[1], v[2]))
    .collect()
}

fn euler_distance(a: (usize, usize, usize), b: (usize, usize, usize)) -> f64 {
    let dx = (a.0 as isize - b.0 as isize) as f64;
    let dy = (a.1 as isize - b.1 as isize) as f64;
    let dz = (a.2 as isize - b.2 as isize) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

        let expected = r#"40"#;

        let result = part1(input, 10);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

        let expected = r#"25272"#;

        let result = part2(input);
        assert_eq!(result, expected);
    }
}
