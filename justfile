set dotenv-load

# Start a new challenge
new-day-bin NUMBER:
    touch src/day{{NUMBER}}.rs
    cargo aoc input -d {{NUMBER}} -y 2022

# Run a submission
run NUMBER:
    cargo aoc -d {{NUMBER}}

bench NUMBER:
    cargo aoc bench -d {{NUMBER}}
