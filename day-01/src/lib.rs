use color_eyre::eyre::eyre;

pub fn part1(input: &str) -> String {
    let input_data = parse_input(input);
    let mut position: isize = 50;
    let mut times_zero = 0;
    for value in input_data {
        position += value % 100;
        if position > 99 {
            position -= 100;
        } else if position < 0 {
            position += 100;
        }
        if position == 0 {
            times_zero += 1;
        }
    }
    times_zero.to_string()
}

pub fn part2(input: &str) -> String {
    todo!("Implement part 2");
}

fn parse_input(input: &str) -> Vec<isize> {
    let parsed_input = input.replace("L", "-").replace("R", "");
    parsed_input
        .lines()
        .map(|line| match line.parse::<isize>() {
            Ok(v) => v,
            Err(e) => panic!("failed to parse line '{}': {}", line, e),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

        let expected = r#"3"#;

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
