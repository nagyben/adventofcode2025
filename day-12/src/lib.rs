use nom::{
    IResult, Parser,
    branch::alt,
    bytes::{complete::tag, take},
    character::complete::{digit1, multispace0, newline, space1},
    multi::{count, separated_list1},
    sequence::separated_pair,
};

pub fn part1(input: &str) -> String {
    let (input, (shapes, regions)) = parse_input(input).unwrap();
    let shapes_area = shapes
        .iter()
        .map(|shape| {
            shape
                .iter()
                .map(|row| row.iter().filter(|c| **c == "#").count())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    let regions_fit = regions
        .iter()
        .map(|region| {
            let region_area = region.area();
            let shapes_area_required = region.shape_requirements.iter().map(|v| v * 9).sum();
            region_area >= shapes_area_required
        })
        .collect::<Vec<bool>>();

    regions_fit.iter().filter(|r| **r).count().to_string()
}

pub fn part2(input: &str) -> String {
    todo!("Implement part 2");
}

type Shape<'a> = Vec<Vec<&'a str>>;

#[derive(Debug)]
struct Region {
    dims: (usize, usize),
    shape_requirements: Vec<usize>,
}

impl Region {
    pub fn area(&self) -> usize {
        self.dims.0 * self.dims.1
    }
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Shape>, Vec<Region>)> {
    let (input, shapes) = separated_list1(newline, parse_shape).parse(input)?;
    let (input, _) = multispace0.parse(input)?;
    let (input, regions) = separated_list1(newline, parse_region).parse(input)?;
    Ok((input, (shapes, regions)))
}

fn parse_shape(input: &str) -> IResult<&str, Shape<'_>> {
    let (input, _) = multispace0.parse(input)?;
    let (input, _) = digit1.parse(input)?;
    let (input, _) = tag(":").parse(input)?;
    let (input, _) = multispace0.parse(input)?;
    let (input, shape) =
        separated_list1(newline, count(alt((tag("#"), tag("."))), 3)).parse(input)?;
    Ok((input, shape))
}

fn parse_region(input: &str) -> IResult<&str, Region> {
    let (input, _) = multispace0.parse(input)?;
    let (input, dims) = separated_pair(
        digit1.map_res(|c: &str| c.parse::<usize>()),
        tag("x"),
        digit1.map_res(|c: &str| c.parse::<usize>()),
    )
    .parse(input)?;

    let (input, _) = tag(":").parse(input)?;
    let (input, _) = multispace0.parse(input)?;

    let (input, shape_requirements) =
        separated_list1(space1, digit1.map_res(|c: &str| c.parse::<usize>())).parse(input)?;

    Ok((
        input,
        Region {
            dims,
            shape_requirements,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

        let expected = r#"2"#;

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
