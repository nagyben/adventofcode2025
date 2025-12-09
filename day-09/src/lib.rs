use geo::{Coord, Intersects, LineString, Polygon, point};
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

    let poly = Polygon::new(
        LineString::from(
            tiles
                .iter()
                .map(|(a, b)| Coord {
                    x: *a as f64,
                    y: *b as f64,
                })
                .collect::<Vec<_>>(),
        ),
        vec![],
    );

    let tile_pairs: Vec<_> = tiles
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, tile_area(a, b)))
        .collect();

    println!("Total tile pairs: {}", tile_pairs.len());

    let valid_tile_pairs: Vec<_> = tile_pairs
        .into_iter()
        .filter(|(a, b, _)| {
            let points = get_points_for_tile_pair(*a, *b);
            points.iter().all(|p| poly.intersects(p))
        })
        .collect();

    println!("Valid tile pairs: {}", valid_tile_pairs.len());

    valid_tile_pairs
        .into_iter()
        .max_by(|a, b| a.2.cmp(&b.2))
        .unwrap()
        .2
        .to_string()
}

fn tile_area(a: Tile, b: Tile) -> usize {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

fn get_points_for_tile_pair(a: Tile, b: Tile) -> Vec<geo::Point> {
    let (x1, y1) = a;
    let (x2, y2) = b;

    vec![
        point!(x:x1 as f64, y:y1 as f64),
        point!(x:x1 as f64, y:y2 as f64),
        point!(x:x2 as f64, y:y1 as f64),
        point!(x:x2 as f64, y:y2 as f64),
    ]
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
        let input = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

        let expected = r#"24"#;

        let result = part2(input);
        assert_eq!(result, expected);
    }
}
