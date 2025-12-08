use color_eyre::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, newline, space0, space1},
    multi::separated_list1,
    sequence::pair,
};
use polars::prelude::*;

pub fn part1(input: &str) -> String {
    let (numbers, operators) = part1::parse_input(input);

    let numbers = vec_to_dataframe(numbers);
    numbers
        .column_iter()
        .zip(operators.iter())
        .map(|(col, operator)| {
            let column_aggregation = match operator {
                Operator::Add(_) => col.sum_reduce().unwrap(),
                Operator::Multiply(_) => col.product().unwrap(),
            };

            match column_aggregation.value() {
                AnyValue::UInt64(v) => *v,
                _ => panic!("Unexpected value type"),
            }
        })
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (numbers, operators) = part2::parse_input(input);
    let numbers = vec_to_dataframe(numbers);
    todo!()
}

/// holds the operator and the column width
#[derive(Debug, PartialEq)]
enum Operator {
    Multiply(usize),
    Add(usize),
}

mod part1 {
    use super::*;

    pub(crate) fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<Operator>) {
        let (input, numbers) = separated_list1(newline, parse_number_row)
            .parse(input)
            .unwrap();

        let operators = parse_operators(input);
        (numbers, operators)
    }

    fn parse_number_row(input: &str) -> IResult<&str, Vec<u64>> {
        let (input, _) = multispace0.parse(input)?;
        separated_list1(space1, digit1.map_res(|d: &str| d.parse::<u64>())).parse(input)
    }
}

mod part2 {
    use nom::{
        bytes::complete::{take, take_till},
        combinator::{eof, rest},
        error::Error,
        multi::many1,
    };

    use super::*;
    pub(crate) fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<Operator>) {
        let operators = parse_operators(input);

        let numbers: Vec<Vec<&str>> = input
            .lines()
            .map(|mut line| {
                let mut number: &str;
                let mut row: Vec<&str> = vec![];
                for operator in operators.iter() {
                    let n = match operator {
                        Operator::Multiply(n) => n,
                        Operator::Add(n) => n,
                    };

                    (line, number) = alt((take::<_, _, nom::error::Error<&str>>(*n), rest))
                        .parse(line)
                        .unwrap();

                    (line, _) = space0::<_, nom::error::Error<&str>>.parse(line).unwrap(); // take the space
                    row.push(number);
                }
                row
            })
            .collect();

        println!("{:?}", numbers);
        todo!()
    }
}

fn parse_operators(input: &str) -> Vec<Operator> {
    let mut row = input.lines().last().unwrap();
    let mut operators = Vec::new();
    let operator_row_length = row.len();

    // There is no trailing whitespace after the last operator,
    // so we have to calculate the padding based on the length of the longest line
    let longest_line = input.lines().max_by(|x, y| x.len().cmp(&y.len())).unwrap();

    while !row.is_empty() {
        let operator: Operator;
        let padding: &str;
        (row, (operator, padding)) = pair(parse_operator, space0).parse(row).unwrap();
        let column_width = if padding.is_empty() {
            longest_line.len() - operator_row_length + 1
        } else {
            padding.len()
        };

        operators.push(match operator {
            Operator::Multiply(_) => Operator::Multiply(column_width),
            Operator::Add(_) => Operator::Add(column_width),
        });
    }
    operators
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    alt((
        tag("*").map(|_| Operator::Multiply(0)),
        tag("+").map(|_| Operator::Add(0)),
    ))
    .parse(input)
}

pub(crate) fn vec_to_dataframe(numbers: Vec<Vec<u64>>) -> DataFrame {
    // Transpose the data: convert rows to columns
    let num_cols = numbers.first().map(|row| row.len()).unwrap_or(0);

    let columns: Vec<Column> = (0..num_cols)
        .map(|col_idx| {
            let series: Series = numbers
                .iter()
                .map(|row| row[col_idx])
                .collect::<Series>()
                .with_name(format!("{}", col_idx).into());
            series.into_column()
        })
        .collect();

    DataFrame::new(columns).expect("Failed to create DataFrame")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"#;

        let expected = r#"4277556"#;

        let result = part1(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"#;

        let expected = r#"3263827"#;

        let result = part2(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_input_part2() {
        let input = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"#;
        let expected_numbers = vec![
            [123, 328, 510, 640],
            [45, 640, 387, 230],
            [6, 980, 215, 314],
        ];

        let expected_operators = vec![
            Operator::Multiply(3),
            Operator::Add(3),
            Operator::Multiply(3),
            Operator::Add(3),
        ];
        let (actual_numbers, actual_operators) = part2::parse_input(input);
        assert_eq!(actual_numbers, expected_numbers);
        assert_eq!(actual_operators, expected_operators);
    }
}
