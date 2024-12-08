defmodule Grid do
  defstruct [:content, :antinodes]

  @type t :: %__MODULE__{
          content: Arrays.array(Arrays.array(char())),
          antinodes: Arrays.array(Arrays.array(boolean()))
        }

  @spec new(Enumerable.t() | list(list(char()))) :: Grid.t()
  def new(input) do
    grid_content =
      input
      |> Enum.map(&Arrays.new(&1, implementation: Arrays.Implementations.ErlangArray))
      |> Arrays.new(implementation: Arrays.Implementations.ErlangArray)

    height = grid_content |> Arrays.size()

    width =
      if height == 0 do
        0
      else
        grid_content[0] |> Arrays.size()
      end

    antinodes =
      1..height
      |> Enum.map(fn _ ->
        Stream.repeatedly(fn -> false end)
        |> Enum.take(width)
        |> Arrays.new(implementation: Arrays.Implementations.ErlangArray)
      end)
      |> Arrays.new(implementation: Arrays.Implementations.ErlangArray)

    %Grid{content: grid_content, antinodes: antinodes}
  end

  @spec width(Grid.t()) :: non_neg_integer()
  def width(grid) do
    if grid |> Grid.height() > 0 do
      grid.content[0] |> Arrays.size()
    else
      0
    end
  end

  @spec height(Grid.t()) :: non_neg_integer()
  def height(%Grid{content: content}) do
    content |> Arrays.size()
  end

  @spec get(Grid.t(), non_neg_integer(), non_neg_integer()) ::
          {:error, String.t()} | {:ok, char()}
  def get(grid, x, y) do
    %Grid{content: content} = grid
    width = grid |> Grid.width()
    height = grid |> Grid.height()

    if x >= width || y >= height do
      {:error, "out of bounds: (#{x}, #{y}) when size is (#{width}, #{height})"}
    else
      {:ok, content[y][x]}
    end
  end

  @spec set_antinode(Grid.t(), non_neg_integer(), non_neg_integer()) ::
          {:error, String.t()} | {:ok, Grid.t()}
  def set_antinode(grid, x, y) do
    %Grid{content: content, antinodes: antinodes} = grid
    width = grid |> Grid.width()
    height = grid |> Grid.height()

    if antinodes[y][x] do
      grid
    else
      if 0 <= x && x < width && 0 <= y && y < height do
        {_, new_antinodes} =
          Access.get_and_update(antinodes, y, fn row ->
            Access.get_and_update(row, x, fn v -> {v, true} end)
          end)

        {:ok, %Grid{content: content, antinodes: new_antinodes}}
      else
        {:error, "out of bounds: (#{x}, #{y}) when size is (#{width}, #{height})"}
      end
    end
  end

  @spec resonate(
          Grid.t(),
          non_neg_integer(),
          non_neg_integer(),
          non_neg_integer(),
          non_neg_integer()
        ) :: [{any(), any()}]
  def resonate(grid, x, y, diff_x, diff_y) do
    width = Grid.width(grid)
    height = Grid.height(grid)

    gcd = gcd(abs(diff_x), abs(diff_y))
    diff_x = div(diff_x, gcd)
    diff_y = div(diff_y, gcd)

    -50..50
    |> Enum.map(&{x + &1 * diff_x, y + &1 * diff_y})
    |> Enum.filter(fn {x, y} -> 0 <= x && x < width && 0 <= y && y < height end)
  end

  @spec antennae_pairs(Grid.t()) :: [
          {{non_neg_integer(), non_neg_integer()}, {non_neg_integer(), non_neg_integer()}}
        ]
  def antennae_pairs(grid) do
    width = grid |> Grid.width()
    height = grid |> Grid.height()

    antennae =
      for x <- 0..(width - 1), y <- 0..(height - 1) do
        with {:ok, val} <- grid |> Grid.get(x, y) do
          cond do
            val != "." -> {val, {x, y}}
            true -> nil
          end
        end
      end
      |> Enum.filter(&is_tuple/1)
      |> Enum.group_by(fn {val, _} -> val end, fn {_, pos} -> pos end)

    antennae
    |> Enum.flat_map(fn {_, positions} ->
      for p1 <- positions, p2 <- positions do
        {p1, p2}
      end
    end)
    |> Enum.filter(fn {{x1, y1}, {x2, y2}} -> x1 < x2 || (x1 == x2 && y1 < y2) end)
  end

  defimpl String.Chars, for: Grid do
    @spec to_string(Grid.t()) :: String.t()
    def to_string(%Grid{content: content, antinodes: antinodes}) do
      content = content |> Enum.map(&Enum.join/1) |> Enum.join("\n")

      antinodes =
        antinodes
        |> Enum.map(fn row ->
          row
          |> Enum.map(
            &if &1 do
              "#"
            else
              "."
            end
          )
        end)
        |> Enum.map(&Enum.join/1)
        |> Enum.join("\n")

      [content, antinodes] |> Enum.join("\n\n")
    end
  end

  @spec gcd(non_neg_integer(), non_neg_integer()) :: non_neg_integer()
  defp gcd(a, b) do
    cond do
      a == 0 || b == 0 -> max(1, max(a, b))
      a > b -> gcd(rem(a, b), b)
      a < b -> gcd(a, rem(b, a))
      a == b -> a
    end
  end
end
