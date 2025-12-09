use itertools::Itertools;

type Tile = (usize, usize);
pub fn part1(input: &str) -> String {
    let tiles = input
        .lines()
        .map(|line| {
            let nums = line
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (nums[0], nums[1])
        })
        .collect::<Vec<Tile>>();

    let tile_pairs: Vec<_> = tiles
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, tile_area(a, b)))
        .collect();

    tile_pairs
        .into_iter()
        .max_by(|a, b| a.2.cmp(&b.2))
        .unwrap()
        .2
        .to_string()
}

pub fn part2(input: &str) -> String {
    todo!("Implement part 2");
}

fn tile_area(a: Tile, b: Tile) -> usize {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

        let expected = r#"50"#;

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
