defmodule Day11 do
  @spec sample() :: binary()
  def sample do
    File.read!("sample.txt")
  end

  @spec input() :: binary()
  def input do
    File.read!("input.txt")
  end

  @spec parse_input(String.t()) :: %{non_neg_integer() => non_neg_integer()}
  def parse_input(input) do
    input
    |> String.split(" ")
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.to_integer/1)
    |> Enum.reduce(%{}, &Map.update(&2, &1, 1, fn old -> old + 1 end))
  end

  def step(map) do
    map
    |> Enum.reduce(%{}, fn {stone, count}, map ->
      nd_even = rem(num_digits(stone), 2) == 0

      case stone do
        0 ->
          Map.update(map, 1, count, fn old -> old + count end)

        v when nd_even ->
          {l, r} = split_digits(v)

          map
          |> Map.update(l, count, fn old -> old + count end)
          |> Map.update(r, count, fn old -> old + count end)

        _ ->
          Map.update(map, stone * 2024, count, fn old -> old + count end)
      end
    end)
  end

  defp num_digits(n) do
    (n + 1) |> :math.log10() |> ceil
  end

  defp split_digits(n) do
    digits = div(num_digits(n), 2)
    factor = 10 |> :math.pow(digits) |> round
    {div(n, factor), rem(n, factor)}
  end
end
