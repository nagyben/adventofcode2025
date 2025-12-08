use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1},
    combinator::map_res,
    sequence::pair,
};

pub(crate) fn parse_input(input: &str) -> ! {
    let operators = parse_operators(input);
    println!("{:?}", operators);
    todo!()
}

/// holds the operator and the column width
#[derive(Debug, PartialEq)]
enum Operator {
    Multiply(usize),
    Add(usize),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_operators() {
        let input = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"#;

        let operators = parse_operators(input);
        let expected = vec![
            Operator::Multiply(3),
            Operator::Add(3),
            Operator::Multiply(3),
            Operator::Add(3),
        ];

        assert_eq!(operators, expected);
    }
}
