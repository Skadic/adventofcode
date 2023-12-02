defmodule Day01 do

  def task1 do
    find_digits = fn chars ->
      chars |> Enum.map(&Integer.parse/1) |> Enum.filter(&(&1 != :error))
    end

    File.read!("../../inputs/day01/input.txt")
    |> String.split()
    |> Enum.map(&String.codepoints/1)
    |> Enum.map(find_digits)
    |> Enum.map(&{List.first(&1), List.last(&1)})
    |> Enum.map(fn {{l, _}, {r, _}} -> l * 10 + r end)
    |> Enum.sum()
  end

  def parse_any_num(line, index) do
    max_len = min(5, String.length(line) - index)

    for len <- 1..max_len do
      token =
        line
        |> String.slice(index, len)

      with true <- String.length(token) == 1,
           {value, _} <- Integer.parse(token),
           true <- value != :error do
        value
      else
        _ ->
          case token do
            "zero" -> 0
            "one" -> 1
            "two" -> 2
            "three" -> 3
            "four" -> 4
            "five" -> 5
            "six" -> 6
            "seven" -> 7
            "eight" -> 8
            "nine" -> 9
            _ -> :invalid
          end
      end
    end
    |> Enum.filter(&(&1 != :invalid))
  end

  def task2 do
    for line <-
          File.read!("../../inputs/day01/input.txt")
          |> String.split() do
      0..(String.length(line) - 1)
      |> Enum.flat_map(&parse_any_num(line, &1))
    end
    |> Enum.map(&{List.first(&1), List.last(&1)})
    |> Enum.map(&(elem(&1, 0) * 10 + elem(&1, 1)))
    |> Enum.sum()
  end
end
