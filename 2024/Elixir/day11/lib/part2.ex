defmodule Day11.Part2 do
  def main(_args \\ []) do
    stones = Day11.input() |> Day11.parse_input()

    1..75
    |> Enum.reduce(stones, fn _, stones -> Day11.step(stones) end)
    |> Enum.reduce(0, fn {_, cnt}, sum -> sum + cnt end)
    |> IO.puts()
  end
end
