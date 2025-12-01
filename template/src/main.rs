use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // package name will be of the format "day-XX"
    // use the cargo package name to determine the day number
    let day: usize = std::env::var("CARGO_PKG_NAME")
        .expect("CARGO_PKG_NAME not set")
        .split('-')
        .next_back()
        .and_then(|s| s.parse::<usize>().ok())
        .expect("Failed to parse day number from crate name");

    let input = util::get_aoc_input(2025, day).await?;

    let part1_result = {{crate_name}}::part1(&input);
    println!("Day {}: Part 1: {:?}", day, part1_result);

    let part2_result = {{crate_name}}::part2(&input);
    println!("Day {}: Part 2: {:?}", day, part2_result);

    Ok(())
}
