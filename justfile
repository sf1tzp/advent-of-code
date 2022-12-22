set dotenv-load

# Display this help
default:
    @just --list --unsorted

# Start a new solution
new DAY:
    @just input {{DAY}}
    touch src/day{{DAY}}.rs

# Run a solution
run DAY:
    cargo aoc -d {{DAY}}

# Benchmark a solution
benchmark DAY:
    cargo aoc bench -d {{DAY}} -g

# Download input (Done automatically with new)
input DAY:
    cargo aoc input -y 2022 -d {{DAY}}

