import gleam/dict
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import glearray
import simplifile

pub fn main() {
  use content <- result.try(
    simplifile.read("input.txt") |> result.replace_error(Nil),
  )

  use #(towels, combos) <- result.try(
    parse_input(content) |> result.replace_error(Nil),
  )

  process(towels, combos)

  Ok(Nil)
}

pub fn parse_input(input: String) -> Result(#(List(String), List(String)), Nil) {
  use #(towels, combos) <- result.try(input |> string.split_once("\n\n"))

  Ok(#(
    towels
      |> string.split(",")
      |> list.map(fn(s) { string.trim(s) })
      |> list.sort(fn(l, r) { int.compare(string.length(l), string.length(r)) }),
    combos |> string.split("\n"),
  ))
}

pub fn process(towels: List(String), combos: List(String)) {
  let possibilities =
    {
      use combo <- list.map(combos)
      use num_combos, i <- list.fold(
        list.range(1, string.length(combo)),
        glearray.new() |> glearray.copy_push(1),
      )

      num_combos_until(num_combos, i, combo, towels)
    }
    |> list.map(fn(l) { glearray.to_list(l) |> list.last() |> result.unwrap(0) })

  let part1 = possibilities |> list.count(fn(v) { v > 0 })
  let part2 = possibilities |> int.sum

  io.println("part 1: " <> int.to_string(part1))
  io.println("part 2: " <> int.to_string(part2))
}

fn num_combos_until(
  num_combos: glearray.Array(Int),
  i: Int,
  combo: String,
  towels: List(String),
) {
  let valid_towels =
    towels
    |> list.take_while(fn(t) { string.length(t) <= i })
    |> list.group(fn(t) { string.length(t) })

  let possibilities =
    valid_towels
    |> dict.map_values(fn(len, towels) {
      case
        list.any(towels, fn(t) {
          string.slice(combo, i - len, len) |> string.starts_with(t)
        })
      {
        True -> num_combos |> glearray.get(i - len) |> result.unwrap(0)
        False -> 0
      }
    })
    |> dict.values
    |> int.sum

  glearray.copy_push(num_combos, possibilities)
}
