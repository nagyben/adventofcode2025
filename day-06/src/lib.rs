use color_eyre::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, newline, space1},
    multi::separated_list1,
};
use polars::prelude::*;

#[derive(Debug)]
enum Operator {
    Multiply,
    Add,
}

pub fn part1(input: &str) -> String {
    let (numbers, operators) = parse_input(input);

    let numbers = vec_to_dataframe(numbers);
    numbers
        .column_iter()
        .zip(operators.iter())
        .map(|(col, operator)| {
            let column_aggregation = match operator {
                Operator::Add => col.sum_reduce().unwrap(),
                Operator::Multiply => col.product().unwrap(),
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
    todo!("Implement part 2");
}

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<Operator>) {
    let (input, numbers) = separated_list1(newline, parse_number_row)
        .parse(input)
        .unwrap();

    let (_, operators) = parse_operator_row.parse(input).unwrap();
    (numbers, operators)
}

fn parse_number_row(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = multispace0.parse(input)?;
    separated_list1(space1, digit1.map_res(|d: &str| d.parse::<u64>())).parse(input)
}

fn parse_operator_row(input: &str) -> IResult<&str, Vec<Operator>> {
    let (input, _) = multispace0.parse(input)?;
    separated_list1(
        space1,
        alt((
            char('*').map(|_| Operator::Multiply),
            char('+').map(|_| Operator::Add),
        )),
    )
    .parse(input)
}

fn vec_to_dataframe(numbers: Vec<Vec<u64>>) -> DataFrame {
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
}
