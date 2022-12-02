set dotenv-load

# Start a new challenge
new-day NUMBER:
    touch src/bin/day-{{NUMBER}}.rs
    curl -o inputs/day-{{NUMBER}}.txt -b "session=$SESSION" https://adventofcode.com/2022/day/{{NUMBER}}/input

# Run a submission
run NUMBER:
    INPUT_PATH="inputs/day-{{NUMBER}}.txt" \
    cargo run --bin day-{{NUMBER}}
