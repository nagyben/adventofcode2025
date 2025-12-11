use std::{collections::HashMap, hash::RandomState};

use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{alpha1, multispace0, newline, space1},
    multi::separated_list1,
};
use pathfinding::{directed::count_paths, prelude::count_paths};
use petgraph::{algo::all_simple_paths, prelude::DiGraphMap};

pub fn part1(input: &str) -> String {
    let neighbours = parse_input(input);
    let num_paths = count_paths(
        "you",
        |next| {
            if *next != "out" {
                neighbours.get(*next).unwrap().clone()
            } else {
                vec![]
            }
        },
        |n| *n == "out",
    );
    num_paths.to_string()
}

pub fn part2(input: &str) -> String {
    let neighbours = parse_input(input);

    // today I learned something about graph theory
    // if you want to count the number of paths from A to B while going through point C,
    // you have to find all the paths from A to C and multiple that with all the paths from C to B
    // if you want to visit multiple intermediate nodes, you have to do this for each segment and
    // multiply the results, and then add all the permutations together
    // so in this example, find all the paths between and multiply together:
    // - svr -> fft
    // - fft -> dac
    // - dac -> out
    //
    // and then add that to the paths for the other permutation:
    // - svr -> dac
    // - dac -> fft
    // - fft -> out
    let p1: usize = [("svr", "fft"), ("fft", "dac"), ("dac", "out")]
        .into_iter()
        .map(|(src, dest)| {
            count_paths(
                src,
                |next| {
                    if *next != "out" {
                        neighbours.get(*next).unwrap().clone()
                    } else {
                        vec![]
                    }
                },
                |n| *n == dest,
            )
        })
        .product();
    let p2: usize = [("svr", "dac"), ("dac", "fft"), ("fft", "out")]
        .into_iter()
        .map(|(src, dest)| {
            count_paths(
                src,
                |next| {
                    if *next != "out" {
                        neighbours.get(*next).unwrap().clone()
                    } else {
                        vec![]
                    }
                },
                |n| *n == dest,
            )
        })
        .product();
    (p1 + p2).to_string()
}

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    separated_list1(newline, parse_line)
        .parse(input)
        .map(|(_, vec)| vec.into_iter().collect())
        .unwrap()
}

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (input, key) = take_until(":").parse(input)?;
    let (input, _) = tag(":").parse(input)?;
    let (input, _) = multispace0.parse(input)?;
    let (input, nodes) = separated_list1(space1, alpha1).parse(input)?;
    Ok((input, (key, nodes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;

        let expected = r#"5"#;

        let result = part1(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

        let expected = r#"2"#;

        let result = part2(input);
        assert_eq!(result, expected);
    }
}
