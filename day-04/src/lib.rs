use itertools::iproduct;

type Grid = Vec<Vec<char>>;

pub fn part1(input: &str) -> String {
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

    let num_rolls_accessible: usize = iproduct!(0..grid[0].len(), 0..grid.len())
        .filter(|(x, y)| grid[*y][*x] == '@')
        .map(|(x, y)| (x as isize, y as isize))
        .filter(|position| position_accessible(&grid, *position))
        .count();

    num_rolls_accessible.to_string()
}

pub fn part2(input: &str) -> String {
    let mut grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

    let mut removed_rolls = 0;
    loop {
        let accessible_positions = get_accessible_positions(&grid);
        removed_rolls += accessible_positions.len();
        if accessible_positions.is_empty() {
            break;
        }
        for (x, y) in accessible_positions {
            grid[y][x] = '.';
        }
    }
    removed_rolls.to_string()
}

fn get_accessible_positions(grid: &Grid) -> Vec<(usize, usize)> {
    let accessible_positions: Vec<(usize, usize)> = iproduct!(0..grid[0].len(), 0..grid.len())
        .filter(|(x, y)| grid[*y][*x] == '@')
        .map(|(x, y)| (x as isize, y as isize))
        .filter(|position| position_accessible(&grid, *position))
        .map(|(x, y)| (x as usize, y as usize))
        .collect();
    accessible_positions
}

fn position_accessible(grid: &Grid, position: (isize, isize)) -> bool {
    let mut surrounding_rolls = 0;
    for (x, y) in iproduct!(
        position.0 - 1..=position.0 + 1,
        position.1 - 1..=position.1 + 1
    ) {
        if x >= 0
            && y >= 0
            && let Some(row) = grid.get(y as usize)
            && let Some(p) = row.get(x as usize)
            && *p == '@'
        {
            surrounding_rolls += 1;
        }
    }
    // we counted the roll itself, so subtract 1
    surrounding_rolls -= 1;
    surrounding_rolls < 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

        let expected = r#"13"#;

        let result = part1(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

        let expected = r#"43"#;

        let result = part2(input);
        assert_eq!(result, expected);
    }
}
