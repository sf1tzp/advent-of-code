set dotenv-load

# Display this help
_default:
    #!/usr/bin/env bash
    set -euo pipefail
    just -ul --list-heading $'Advent of Code 2022 Commands\n'

# Login to Advent of Code (Use AOC_SESSION_TOKEN environment variable)
login:
    #!/usr/bin/env bash
    set -euo pipefail

    # Check if already logged in
    if [ ! -z $(cargo aoc credentials | sed -E 's/Current credentials: //g') ]; then
        exit 0
    fi

    # Check for session token
    if [ -z "$AOC_SESSION_TOKEN" ]; then
        echo "AOC_SESSION_TOKEN is not set"
        exit 1
    fi

    cargo aoc credentials -s $AOC_SESSION_TOKEN > /dev/null
    echo "Logged in to Advent of Code"


# Start a new solution for day number DAY
new DAY:
    #!/usr/bin/env bash
    if [ ! -f "input/2022/day{{DAY}}.txt" ]; then
        just _get-input {{DAY}}
    fi
    touch src/day{{DAY}}.rs
    sed -i "s/\/\/ pub mod day{{DAY}};/pub mod day{{DAY}};/" src/lib.rs

# Run the solution for day number DAY
run DAY: _ensure-input
    @cargo aoc -d {{DAY}}

# Run a solution with a sample input file (place in `samples/2022/day{{DAY}}.txt`)
run-sample DAY: _ensure-input
    #!/usr/bin/env bash
    file="samples/2022/day{{DAY}}.txt"
    if [ ! -f "$file" ]; then
        echo "Sample input file $file not found"
        exit
    fi
    @cargo aoc -i samples/2022/day{{DAY}}.txt -d {{DAY}}

# Benchmark the solution for day number DAY
run-benchmark DAY: _ensure-input
    @cargo aoc bench -d {{DAY}} -g

# Get an input file
_get-input DAY: login
    @cargo aoc input -y 2022 -d {{DAY}}

# All input must be present to compile
_ensure-input: login
    #!/usr/bin/env bash
    for file in src/day*.rs; do
        day=$(echo "$file" | sed -E 's/[^0-9]//g')
        if [ ! -f "input/2022/day$day.txt" ]; then
            just _get-input "$day"
        fi
    done
