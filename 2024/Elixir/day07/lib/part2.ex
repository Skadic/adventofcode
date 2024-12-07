defmodule Day07.Part2 do
  @spec main() :: :ok
  @spec main(any()) :: :ok
  def main(_args \\ []) do
    Day07.input()
    |> Day07.parse_input()
    |> Enum.filter(fn %Equation{target: target, operands: [head | tail]} ->
      solve_equation(
        target,
        tail,
        head
      )
    end)
    |> Enum.map(& &1.target)
    |> Enum.sum()
    |> IO.puts()
  end

  @spec solve_equation(non_neg_integer(), list(non_neg_integer()), non_neg_integer()) :: boolean()
  defp solve_equation(target, [], acc) do
    acc == target
  end

  @spec solve_equation(non_neg_integer(), list(non_neg_integer()), non_neg_integer()) :: boolean()
  defp solve_equation(target, [head | tail], acc) do
    [&concat_num/2, &+/2, &*/2]
    |> Enum.map(& &1.(acc, head))
    |> Enum.map(&solve_equation(target, tail, &1))
    |> Enum.any?()
  end

  defp concat_num(l, r) do
    r_digits = (r + 1) |> :math.log10() |> ceil()
    l * :math.pow(10, r_digits) + r
  end
end
