defmodule Day08 do
  import Grid

  @spec sample() :: binary()
  def sample do
    File.read!("sample.txt")
  end

  @spec sample2() :: binary()
  def sample2 do
    File.read!("sample2.txt")
  end

  @spec input() :: binary()
  def input do
    File.read!("input.txt")
  end

  @spec parse_input(String.t()) :: Grid.t()
  def parse_input(input) do
    input |> String.split("\n") |> Enum.map(&String.graphemes/1) |> Grid.new()
  end
end
