use cached::proc_macro::io_cached;
use color_eyre::{Result, eyre::eyre};

/// Fetches the Advent of Code input for the given year and day.
/// Caches the input on disk to avoid redundant network requests.
/// Requires the AOC_SESSION environment variable to be set with a valid session cookie.
/// automatically reads AOC_SESSION from .env file if present.
#[io_cached(
    disk = true,
    disk_dir = "./.aoc_cache",
    map_error = r##"|e| eyre!(format!("{:?}", e))"##,
    convert = r##"{ format!("{}:{}", year, day) }"##,
    key = "String",
    sync_to_disk_on_cache_change = true
)]
pub async fn get_aoc_input(year: usize, day: usize) -> Result<String> {
    println!("Fetching input for year {}, day {}", year, day);
    let _ = dotenvy::dotenv();
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .header(
            "Cookie",
            format!(
                "session={}",
                std::env::var("AOC_SESSION").expect("AOC_SESSION not set")
            ),
        )
        .header("Content-Type", "text/plain")
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;
    if !status.is_success() {
        return Err(eyre!("Failed to fetch input: HTTP {}: {}", status, body));
    }
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_aoc_input() -> Result<()> {
        let result = get_aoc_input(2024, 1).await?;
        println!("{:?}", result);
        Ok(())
    }
}
