# 🎄 Advent of Code 🖥 2022 🎁

It's that time of year again! 🎁 🖥 🎄

## Project Set Up

This year I'll mainly be using rust. `src/lib.rs` contains any re-usable bits such as reading input from a file. Each solution will then be written in a in `src/bin/day-##.rs` file.

In the spirit of Rustmas I've added two `just` targets here to help speed things along:

- `new-day ##` creates a new file in `src/bin`. Grab the `session` cookie from adventofcode.com and save it in a `.env` file so that `just` can retreive the day's input.
- `run ##` sources the input file and runs the binary for a given day.
