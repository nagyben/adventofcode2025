pub fn part1(input: &str) -> String {
    let ranges = input
        .split(',')
        .filter_map(|range| {
            let mut parts = range.split('-');
            let from = parts.next()?.trim().parse::<usize>().ok()?;
            let to = parts.next()?.trim().parse::<usize>().ok()?;
            Some((from, to))
        })
        .collect::<Vec<(usize, usize)>>();

    ranges
        .iter()
        .flat_map(|(from, to)| part1::invalid_ids_in_range(*from, *to))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let ranges = input
        .split(',')
        .filter_map(|range| {
            let mut parts = range.split('-');
            let from = parts.next()?.trim().parse::<usize>().ok()?;
            let to = parts.next()?.trim().parse::<usize>().ok()?;
            Some((from, to))
        })
        .collect::<Vec<(usize, usize)>>();

    ranges
        .iter()
        .flat_map(|(from, to)| part2::invalid_ids_in_range(*from, *to))
        .sum::<usize>()
        .to_string()
}

mod part1 {
    pub(crate) fn invalid_ids_in_range(from: usize, to: usize) -> Vec<usize> {
        (from..=to).filter(|&id| check_invalid_id(id)).collect()
    }

    pub(crate) fn check_invalid_id(id: usize) -> bool {
        let num_digits = id.to_string().len();
        if !num_digits.is_multiple_of(2) {
            // odd number of digits cannot be invalid
            return false;
        }

        let first_half = id.to_string()[..num_digits / 2].to_string();
        let second_half = id.to_string()[num_digits / 2..].to_string();
        first_half == second_half
    }
}

mod part2 {
    pub(crate) fn invalid_ids_in_range(from: usize, to: usize) -> Vec<usize> {
        (from..=to).filter(|&id| check_invalid_id(id)).collect()
    }

    // The idea behind this approach is that if a string s can be constructed by repeating a substring,
    // then concatenating two copies of s together and removing the first and last character would
    // still contain s as a substring.
    //
    // Example:
    //
    // Given the string "abab":
    // Concatenate to get "abababab".
    // Remove first and last characters to get "bababa".
    // Check if "abab" is present in "bababa" - It is. Hence, return true.
    pub(crate) fn check_invalid_id(id: usize) -> bool {
        let double_string = format!("{}{}", id, id);
        double_string[1..double_string.len() - 1].contains(&format!("{}", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_part1 {
        use super::part1;
        use super::part1::*;
        use rstest::rstest;

        #[rstest]
        #[case(11)]
        #[case(22)]
        #[case(99)]
        #[case(1010)]
        #[case(1188511885)]
        #[case(222222)]
        #[case(446446)]
        #[case(38593859)]
        fn test_check_invalid_id(#[case] id: usize) {
            assert!(check_invalid_id(id));
        }

        // 11-22 has two invalid IDs, 11 and 22.
        // 95-115 has one invalid ID, 99.
        // 998-1012 has one invalid ID, 1010.
        // 1188511880-1188511890 has one invalid ID, 1188511885.
        // 222220-222224 has one invalid ID, 222222.
        // 1698522-1698528 contains no invalid IDs.
        // 446443-446449 has one invalid ID, 446446.
        // 38593856-38593862 has one invalid ID, 38593859.
        // The rest of the ranges contain no invalid IDs.
        #[rstest]
        #[case(11, 22, vec![11, 22])]
        #[case(95, 115, vec![99])]
        #[case(998, 1012, vec![1010])]
        #[case(1188511880, 1188511890, vec![1188511885])]
        #[case(222220, 222224, vec![222222])]
        #[case(1698522, 1698528, vec![])]
        #[case(446443, 446449, vec![446446])]
        #[case(38593856, 38593862, vec![38593859])]
        #[case(565653, 565659, vec![])]
        #[case(824824821, 824824827, vec![])]
        #[case(2121212118, 2121212124, vec![])]
        fn test_invalid_ids_in_range(
            #[case] from: usize,
            #[case] to: usize,
            #[case] expected: Vec<usize>,
        ) {
            assert_eq!(invalid_ids_in_range(from, to), expected);
        }

        #[test]
        fn test_part1() {
            let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"#;

            let expected = r#"1227775554"#;

            let result = part1(input);
            assert_eq!(result, expected);
        }
    }

    mod test_part2 {
        use super::part2;
        use super::part2::*;
        use rstest::rstest;

        #[rstest]
        #[case(11)]
        #[case(22)]
        #[case(99)]
        #[case(111)]
        #[case(999)]
        #[case(1010)]
        #[case(1188511885)]
        #[case(222222)]
        #[case(446446)]
        #[case(38593859)]
        #[case(565656)]
        #[case(824824824)]
        #[case(2121212121)]
        #[case(12341234)]
        #[case(123123123)]
        #[case(1212121212)]
        #[case(1111111)]
        fn test_check_invalid_id(#[case] id: usize) {
            assert!(check_invalid_id(id));
        }

        // 11-22 still has two invalid IDs, 11 and 22.
        // 95-115 now has two invalid IDs, 99 and 111.
        // 998-1012 now has two invalid IDs, 999 and 1010.
        // 1188511880-1188511890 still has one invalid ID, 1188511885.
        // 222220-222224 still has one invalid ID, 222222.
        // 1698522-1698528 still contains no invalid IDs.
        // 446443-446449 still has one invalid ID, 446446.
        // 38593856-38593862 still has one invalid ID, 38593859.
        // 565653-565659 now has one invalid ID, 565656.
        // 824824821-824824827 now has one invalid ID, 824824824.
        // 2121212118-2121212124 now has one invalid ID, 2121212121.
        #[rstest]
        #[case(11, 22, vec![11, 22])]
        #[case(95, 115, vec![99, 111])]
        #[case(998, 1012, vec![999, 1010])]
        #[case(1188511880, 1188511890, vec![1188511885])]
        #[case(222220, 222224, vec![222222])]
        #[case(1698522, 1698528, vec![])]
        #[case(446443, 446449, vec![446446])]
        #[case(38593856, 38593862, vec![38593859])]
        #[case(565653, 565659, vec![565656])]
        #[case(824824821, 824824827, vec![824824824])]
        #[case(2121212118, 2121212124, vec![2121212121])]
        fn test_invalid_ids_in_range(
            #[case] from: usize,
            #[case] to: usize,
            #[case] expected: Vec<usize>,
        ) {
            assert_eq!(invalid_ids_in_range(from, to), expected);
        }

        #[test]
        fn test_part2() {
            let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"#;

            let expected = r#"4174379265"#;

            let result = part2(input);
            assert_eq!(result, expected);
        }
    }
}
