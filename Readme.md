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


## Benchmarks

`cargo-aoc` benchmarks `generator` functions, which parse the project input, and `solution` functions, which contain the actual problem solving logic.

| Day | Generator | Part 1 | Part 2 |
| --- | --- | --- | ---|
| 1 | 65.570 us | 1.8305 ns | 1.8369 ns |
| 2 | N/A | 103.17 us | 93.106 us |
| 3 | 32.975 us | 144.37 us | 146.60 us |
| 4 | 109.79 us | 626.89 ns | 511.09 ns |
| 5 | 163.05 us | 3.8378 us | 14.640 us |
| 6 | N/A | 97.679 us | 198.96 us |
| 7 | | | |
| 8 | 311.41 us | 271.94 us | 1.7155 ms |
| 9 | | | |
| 10 | | | |
| 11 | 1.0665 ms | 30.342 us | 13.337 ms |
