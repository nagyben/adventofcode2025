# Advent of Code 2025 solutions

## Set AOC_SESSION envvar

The solutions will automatically fetch the inputs for each puzzle from the AoC website.

To do this, you need to set the `AOC_SESSION` environment variable with your session cookie from the AoC website.

Open the developer tools in your browser, open the AoC calendar (make sure you are logged in), and copy the value of the `Cookie: session={{cookie}}` cookie header.

You can store this value in a `.env` file in the root of the repo, or by setting the envvar when running a day's solution.

## Run the solution for a given day

```sh
cargo run -p day-XX
```

with AOC_SESSION:

```sh
AOC_SESSION="<your session cookie>" cargo run -p day-XX
```

## Starting a new day

To generate a cargo package for a new day, run the following command:

```sh
cargo generate -p template --name day-XX
```
