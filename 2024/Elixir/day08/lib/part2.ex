defmodule Day08.Part2 do
  def main(_args \\ []) do
    grid = Day08.input() |> Day08.parse_input()

    antennae_pairs = Grid.antennae_pairs(grid)

    final_grid =
      antennae_pairs
      |> Enum.reduce(grid, fn {{x1, y1}, {x2, y2}}, grid ->
        {diff_x, diff_y} = {x2 - x1, y2 - y1}

        grid
        |> Grid.resonate(x1, y1, diff_x, diff_y)
        |> Enum.reduce(grid, fn {x, y}, grid ->
          case Grid.set_antinode(grid, x, y) do
            {:ok, new_grid} -> new_grid
            _ -> grid
          end
        end)
      end)

    num_antinodes =
      final_grid
      |> then(fn %Grid{antinodes: antinodes} ->
        antinodes |> Enum.flat_map(& &1) |> Enum.filter(& &1) |> Enum.count()
      end)

    final_grid |> IO.puts()
    num_antinodes |> IO.puts()
  end

end
