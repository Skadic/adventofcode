defmodule Day02 do
  @moduledoc """
  Documentation for `Day02`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Day02.hello()
      :world

  """
  def hello do
    :world
  end

  @spec parse_draw(String.t()) :: %Cubes{}
  def parse_draw(draw) do
    draw
    |> String.trim()
    |> String.split(",")
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.split/1)
    |> Enum.map(fn [count, color] ->
      {String.to_atom(color), elem(Integer.parse(count), 0)}
    end)
    |> Cubes.from_map()
  end

  @spec parse_game(String.t()) :: list(%Cubes{})
  def parse_game(game) do
    game
    |> String.trim()
    |> String.split(";")
    |> Enum.map(&Day02.parse_draw/1)
  end

  @spec parse_input() :: list(list(%Cubes{}))
  def parse_input do
    File.read!("../../inputs/day02/input.txt")
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&String.split(&1, ":"))
    |> Enum.map(fn [_ | elem] -> elem end)
    |> Enum.map(&List.first/1)
    |> Enum.map(&Day02.parse_game/1)
  end

  def part1 do
    parse_input()
    |> Enum.map(fn game -> List.foldl(game, %Cubes{}, &Cubes.max_col/2) end)
    |> Enum.zip(1..100_000)
    |> Enum.filter(&Cubes.is_possible(elem(&1, 0), %Cubes{red: 12, green: 13, blue: 14}))
    |> Enum.map(&elem(&1, 1))
    |> Enum.sum()
  end

  def part2 do
    parse_input()
    |> Enum.map(fn game -> List.foldl(game, %Cubes{}, &Cubes.max_col/2) end)
    |> Enum.map(&Cubes.power/1)
    |> Enum.sum()
  end
end
