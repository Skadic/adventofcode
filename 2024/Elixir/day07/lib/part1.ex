defmodule Day07.Part1 do
  @spec main() :: :ok
  @spec main(any()) :: :ok
  def main(_args \\ []) do
    Day07.sample()
    |> Day07.parse_input()
    |> Enum.filter(fn %Equation{target: target, operands: operands} ->
      solve_equation(target, operands)
    end)
    |> Enum.map(&(&1.target))
    |> Enum.sum()
    |> IO.puts()
  end

  @spec solve_equation(non_neg_integer(), list(non_neg_integer())) :: boolean()
  defp solve_equation(target, remaining) do
    current = List.last(remaining)
    num_remaining = Enum.count(remaining)

    if num_remaining == 1 do
      current == target
    else
      (rem(target, current) == 0 &&
         solve_equation(div(target, current), Enum.slice(remaining, 0..(num_remaining - 2)))) ||
        solve_equation(target - current, Enum.slice(remaining, 0..(num_remaining - 2)))
    end
  end
end
