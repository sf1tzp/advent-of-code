# [🎄 Advent of Code 🖥 2022 🎁][advent-of-code-link]

It's that time of year again! 🎁 🖥 🎄

## Log In

You must set the `AOC_SESSION_TOKEN` environment variable in order to download input. You can find your session token after logging in to the [Advent Of Code website][advent-of-code-link]. Open the inspector, and copy the value of the `session` cookie. Then run

`export AOC_SESSION_TOKEN="<session cookie value>"`

## Commands

This project uses [`cargo-aoc`][cargo-aoc-link] to run and benchmark solutions. The included [`justfile`][just-link] contains shortcuts for most commands in `bash`.

```
Advent of Code 2022 Commands
    login             # Login to Advent of Code (Use AOC_SESSION_TOKEN environment variable)
    new DAY           # Start a new solution for day number DAY
    run DAY           # Run the solution for day number DAY
    run-sample DAY    # Run a solution with a sample input file (place in `samples/2022/day{{DAY}}.txt`)
    run-benchmark DAY # Benchmark the solution for day number DAY
```

[advent-of-code-link]: https://adventofcode.com/2022/
[cargo-aoc-link]: https://github.com/gobanos/cargo-aoc
[just-link]: https://github.com/casey/just
