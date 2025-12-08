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
pub(crate) enum Operator {
    Multiply,
    Add,
}

pub(crate) fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<Operator>) {
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
