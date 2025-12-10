use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, multispace0, multispace1, newline},
    multi::{many0, separated_list1},
    sequence::delimited,
};

#[derive(Debug, Eq, Clone)]
struct Machine {
    indicator_lights_start: Vec<bool>,
    indicator_lights_goal: Vec<bool>,
    wiring_schematics: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

impl Machine {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, indicator_lights) = delimited(
            char('['),
            many0(alt((
                char('#').map_res(|_| Ok::<bool, nom::error::Error<&str>>(true)),
                char('.').map_res(|_| Ok::<bool, nom::error::Error<&str>>(false)),
            ))),
            char(']'),
        )
        .parse(input)?;

        let (input, _) = multispace0.parse(input)?;

        let (input, wiring_schematics) = separated_list1(
            multispace1,
            delimited(
                char('('),
                separated_list1(char(','), nom::character::complete::u32.map(|n| n as usize)),
                char(')'),
            ),
        )
        .parse(input)?;

        let (input, _) = multispace0.parse(input)?;

        let (input, joltage_requirements) = delimited(
            char('{'),
            separated_list1(char(','), nom::character::complete::u32.map(|n| n as usize)),
            char('}'),
        )
        .parse(input)?;

        Ok((
            input,
            Self {
                indicator_lights_start: vec![false; indicator_lights.len()],
                indicator_lights_goal: indicator_lights,
                wiring_schematics,
                joltage_requirements,
            },
        ))
    }
}

impl PartialEq for Machine {
    fn eq(&self, other: &Self) -> bool {
        self.indicator_lights_start == other.indicator_lights_start
    }
}

pub fn push_button(
    mut indicator_lights: Vec<bool>,
    wiring_schematics: &[Vec<usize>],
    button_idx: usize,
) -> Vec<bool> {
    let idxs = &wiring_schematics[button_idx];
    for idx in idxs {
        indicator_lights[*idx] = !indicator_lights[*idx];
    }
    indicator_lights
}

pub fn part1(input: &str) -> String {
    let (_, machines) = separated_list1(newline, Machine::parse)
        .parse(input)
        .unwrap();

    machines
        .iter()
        .map(|machine| {
            // For each machine we perform a breadth-first search (BFS) to find the shortest
            // sequence of button presses to reach the goal state
            pathfinding::prelude::bfs(
                &machine.indicator_lights_start,
                |indicator_lights| {
                    (0..machine.wiring_schematics.len())
                        .map(|button_idx| {
                            push_button(
                                indicator_lights.clone(),
                                &machine.wiring_schematics,
                                button_idx,
                            )
                        })
                        .collect::<Vec<_>>()
                },
                |state| machine.indicator_lights_goal == *state,
            )
            .unwrap()
            .len()
                - 1
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    todo!("Implement part 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

        let expected = r#"7"#;

        let result = part1(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = r#""#;

        let expected = r#""#;

        let result = part2(input);
        assert_eq!(result, expected);
    }
}
