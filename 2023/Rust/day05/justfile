[private]
alias p:=part
[private]
alias b:=bench
[private]
alias i:=inputs
alias load:=inputs
[private]
alias l:=inputs

today := `date "+%-d"`
current_year := `date "+%Y"`

# List all targets
_list:
  just --list --justfile {{justfile()}}

# Run part
part nr: 
  cargo test --release part{{nr}} -- --nocapture

# Benchmark part
bench nr:
  cargo bench --bench part{{nr}}

# Download the input for a given day and year.
inputs day=today year=current_year:
  curl -o input.txt --cookie "session=$AOC_SESSION" https://adventofcode.com/{{year}}/day/{{day}}/input
