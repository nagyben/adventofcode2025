use pathfinding::prelude::count_paths;
use std::collections::HashSet;

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);
pub fn part1(input: &str) -> String {
    let mut grid = parse_grid(input);
    let start_position = find_start_position(&grid);
    let mut split_starts: Vec<Position> = vec![start_position];
    let mut cache: HashSet<Position> = HashSet::new();
    let mut split_positions: HashSet<Position> = HashSet::new();
    let (width, height) = (grid[0].len(), grid.len());

    while let Some((x_start, y_start)) = split_starts.pop() {
        if !cache.contains(&(x_start, y_start)) {
            let mut y = y_start;
            let x = x_start;
            while x < width - 1 && y < height - 1 {
                y += 1;
                if grid[y][x] == '^' {
                    if !split_starts.contains(&(x + 1, y)) {
                        split_starts.push((x + 1, y));
                    }

                    if !split_starts.contains(&(x - 1, y)) {
                        split_starts.push((x - 1, y));
                    }
                    cache.insert((x_start, y_start));
                    split_positions.insert((x, y));
                    break;
                }
                grid[y][x] = '|';
            }
        }
    }
    // print_grid(&grid);
    split_positions.len().to_string()
}

fn print_grid(grid: &Grid) {
    let printed = grid
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>();
    for row in printed.iter() {
        println!("{:?}", row);
    }
}

pub fn part2(input: &str) -> String {
    let grid = parse_grid(input);
    let start_position = find_start_position(&grid);
    count_paths(
        start_position,
        |&(x, y)| {
            if grid[y][x] == '^' {
                vec![(x + 1, y), (x - 1, y)]
            } else {
                vec![(x, y + 1)]
            }
        },
        |&(_, y)| y >= grid.len(),
    )
    .to_string()
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_start_position(grid: &Grid) -> Position {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (x, y)))
        .expect("start position not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

        let expected = r#"21"#;

        let result = part1(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

        let expected = r#"40"#;

        let result = part2(input);
        assert_eq!(result, expected);
    }
}
