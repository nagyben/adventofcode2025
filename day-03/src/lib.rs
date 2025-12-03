pub fn part1(input: &str) -> String {
    let max_joltage: usize = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|batteries| part1::max_joltage_for_bank(batteries))
        .sum();
    max_joltage.to_string()
}

pub fn part2(input: &str) -> String {
    let max_joltage: usize = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|batteries| part2::max_joltage_for_bank(batteries))
        .sum();
    max_joltage.to_string()
}

mod part1 {
    pub fn max_joltage_for_bank(batteries: &[usize]) -> usize {
        // find max left value first
        let mut max_left = 0;
        let mut max_left_index = 0;
        for (i, battery) in batteries[..batteries.len() - 1].iter().enumerate() {
            if *battery > max_left {
                max_left = *battery;
                max_left_index = i;
            }
        }

        // find right max value next
        let mut max_right = batteries[max_left_index + 1];
        for battery in batteries[max_left_index + 1..batteries.len()].iter() {
            if *battery > max_right {
                max_right = *battery;
            }
        }
        max_left * 10 + max_right
    }
}

mod part2 {
    type Battery = usize;
    type Index = usize;
    const NUM_BATTERIES: usize = 12;

    pub fn max_joltage_for_bank(batteries: &[usize]) -> usize {
        let bank_size = batteries.len();
        let mut left_index = 0;
        let mut right_index = bank_size - NUM_BATTERIES;
        let mut chosen_batteries: Vec<usize> = vec![];

        while right_index < bank_size {
            let window = &batteries[left_index..=right_index];
            let (battery_index, battery) = find_biggest_leftmost_battery_in_window(window);
            chosen_batteries.push(battery);
            left_index += 1 + battery_index;
            right_index += 1;
        }
        chosen_batteries
            .iter()
            .fold(0, |acc, battery| acc * 10 + battery)
    }

    fn find_biggest_leftmost_battery_in_window(window: &[usize]) -> (Index, Battery) {
        let mut max_left = 0;
        let mut max_left_index = 0;
        for (i, battery) in window[..window.len()].iter().enumerate() {
            if *battery > max_left {
                max_left = *battery;
                max_left_index = i;
            }
        }
        (max_left_index, max_left)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_part1 {
        use super::part1;
        use rstest::rstest;

        #[rstest]
        #[case("987654321111111", 98)]
        #[case("811111111111119", 89)]
        #[case("234234234234278", 78)]
        #[case("818181911112111", 92)]
        fn test_max_joltage_for_bank(#[case] batteries: &str, #[case] expected: usize) {
            let battery_bank = batteries
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>();
            let result = part1::max_joltage_for_bank(&battery_bank);
            assert_eq!(result, expected);
        }

        #[test]
        fn test_part1() {
            let input = r#"987654321111111
811111111111119
234234234234278
818181911112111
"#;

            let expected = r#"357"#;

            let result = part1(input);
            assert_eq!(result, expected);
        }
    }

    mod test_part2 {
        use super::part2;
        use rstest::rstest;

        #[rstest]
        #[case("987654321111111", 987654321111)]
        #[case("811111111111119", 811111111119)]
        #[case("234234234234278", 434234234278)]
        #[case("818181911112111", 888911112111)]
        fn test_max_joltage_for_bank(#[case] batteries: &str, #[case] expected: usize) {
            let battery_bank = batteries
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>();
            let result = part2::max_joltage_for_bank(&battery_bank);
            assert_eq!(result, expected);
        }

        #[test]
        fn test_part2() {
            let input = r#"987654321111111
811111111111119
234234234234278
818181911112111
"#;

            let expected = r#"3121910778619"#;

            let result = part2(input);
            assert_eq!(result, expected);
        }
    }
}
