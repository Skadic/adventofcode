defmodule Day08.Part1 do
  def main(_args \\ []) do
    grid = Day08.input() |> Day08.parse_input()
    w = Grid.width(grid)
    h = Grid.height(grid)

    antennae =
      for x <- 0..(w - 1), y <- 0..(h - 1) do
        with {:ok, val} <- grid |> Grid.get(x, y) do
          cond do
            val != "." -> {val, {x, y}}
            true -> nil
          end
        end
      end
      |> Enum.filter(&is_tuple/1)
      |> Enum.group_by(fn {val, _} -> val end, fn {_, pos} -> pos end)

    antennae_pairs =
      antennae
      |> Enum.flat_map(fn {_, positions} ->
        for p1 <- positions, p2 <- positions do
          {p1, p2}
        end
      end)
      |> Enum.filter(fn {p1, p2} -> p1 != p2 end)

    final_grid =
      antennae_pairs
      |> Enum.reduce(grid, fn {{x1, y1}, {x2, y2}}, grid ->
        {diff_x, diff_y} = {x2 - x1, y2 - y1}

        IO.puts("#{x1 - diff_x}, #{y1 - diff_y} und #{x2 + diff_x}, #{y2 + diff_y} ")

        grid =
          case Grid.set_antinode(grid, x2 + diff_x, y2 + diff_y) do
            {:ok, new_grid} -> new_grid
            _ -> grid
          end

        grid =
          case Grid.set_antinode(grid, x1 - diff_x, y1 - diff_y) do
            {:ok, new_grid} -> new_grid
            _ -> grid
          end

        grid
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
