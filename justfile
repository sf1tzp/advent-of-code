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
    if [ ! -z "$(cargo aoc credentials | sed -E 's/Current credentials: //g')" ]; then
        exit 0
    fi

    # Check for session token
    if [ -z "$AOC_SESSION_TOKEN" ]; then
        echo "AOC_SESSION_TOKEN is not set"
        exit 1
    fi

    cargo aoc credentials $AOC_SESSION_TOKEN > /dev/null
    echo "Logged in to Advent of Code"


# Start a new solution for day number DAY
new YEAR DAY: (_new_cargo_project YEAR)
    #!/usr/bin/env bash
    pushd {{YEAR}}
    if [ ! -f "input/day{{DAY}}.txt" ]; then
        just _get-input {{YEAR}} {{DAY}}
    fi
    touch src/day{{DAY}}.rs
    sed -i "s/\/\/ pub mod day{{DAY}};/pub mod day{{DAY}};/" src/lib.rs
    popd

# Run the solution for day number DAY
run YEAR DAY: (_ensure-input YEAR)
    #!/usr/bin/env bash
    pushd {{YEAR}}
    cargo aoc -d {{DAY}}
    popd

# Run a solution with a sample input file (place in `samples/2022/day{{DAY}}.txt`)
run-sample YEAR DAY: (_ensure-input YEAR)
    #!/usr/bin/env bash
    pushd {{YEAR}}
    file="samples/day{{DAY}}.txt"
    if [ ! -f "$file" ]; then
        echo "Sample input file $file not found"
        exit
    fi
    cargo aoc -i samples/day{{DAY}}.txt -d {{DAY}}
    popd

# Benchmark the solution for day number DAY
run-benchmark YEAR DAY: (_ensure-input YEAR)
    #!/usr/bin/env bash
    pushd {{YEAR}}
    cargo aoc bench -d {{DAY}} -g
    popd

# Get an input file
_get-input YEAR DAY: login
    #!/usr/bin/env bash
    pushd {{YEAR}}
    cargo aoc input -y {{YEAR}} -d {{DAY}}
    popd

# All input must be present to compile
_ensure-input YEAR: login
    #!/usr/bin/env bash
    set -euo pipefail
    pushd {{YEAR}}
    for file in src/day*.rs; do
        day=$(echo "$file" | sed -E 's/[^0-9]//g')
        if [ ! -f "input/{{YEAR}}/day$day.txt" ]; then
            just _get-input {{YEAR}} "$day"
        fi
    done
    popd

_new_cargo_project YEAR:
    #!/usr/bin/env bash
    if [ ! -d "{{YEAR}}" ]; then
        cargo new advent_of_code_{{YEAR}} --vcs=none
        mv advent_of_code_{{YEAR}} {{YEAR}}
        echo "Created {{YEAR}} project"
        echo "Don't forget to configure main.rs and lib.rs for cargo-aoc"
    fi
