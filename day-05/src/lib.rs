use std::cmp::{max, min};

type FreshIngredientRange = (usize, usize);
type Ingredient = usize;

pub fn part1(input: &str) -> String {
    let (fresh_ingredient_ranges, available_ingredients) = parse_input(input);
    available_ingredients
        .iter()
        .filter(|ingredient| {
            fresh_ingredient_ranges
                .iter()
                .any(|(range_min, range_max)| {
                    **ingredient >= *range_min && **ingredient <= *range_max
                })
        })
        .count()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (mut fresh_ingredient_ranges, _) = parse_input(input);

    fresh_ingredient_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut current_index = 0;

    while current_index < fresh_ingredient_ranges.len() {
        let current_element = fresh_ingredient_ranges[current_index];
        if let Some(next_element) = fresh_ingredient_ranges.get(current_index + 1) {
            if current_element.0 <= next_element.0 && next_element.0 <= current_element.1 {
                // ranges overlap (because they are sorted)
                fresh_ingredient_ranges[current_index] = (
                    min(current_element.0, next_element.0),
                    max(current_element.1, next_element.1),
                );

                fresh_ingredient_ranges.remove(current_index + 1);
            } else {
                current_index += 1
            }
        } else {
            break;
        }
    }

    fresh_ingredient_ranges
        .iter()
        .map(|(min, max)| max - min + 1)
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &str) -> (Vec<FreshIngredientRange>, Vec<Ingredient>) {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let ranges_section = sections.first().unwrap_or(&"");
    let ingredients_section = sections.last().unwrap_or(&"");

    let ranges = ranges_section
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse::<usize>().ok()?;
                let end = parts[1].parse::<usize>().ok()?;
                Some((start, end))
            } else {
                None
            }
        })
        .collect();

    let ingredients = ingredients_section
        .lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .collect();

    (ranges, ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

        let expected = r#"3"#;

        let result = part1(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

        let expected = r#"14"#;

        let result = part2(input);
        assert_eq!(result, expected);
    }
}
