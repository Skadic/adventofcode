defmodule Day07 do
  import Equation

  @spec sample() :: binary()
  def sample do
    File.read!("sample.txt")
  end

  @spec input() :: binary()
  def input do
    File.read!("input.txt")
  end

  @moduledoc """
  Documentation for `Day07`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Day07.hello()
      :world

  """
  @spec parse_input(String.t()) :: list(Equation.t())
  def parse_input(input) do
    input
    |> String.split("\n")
    |> Enum.map(&String.trim/1)
    |> Enum.filter(&(&1 != ""))
    |> Enum.map(&String.split(&1, ": "))
    |> Enum.map(fn split ->
      target = split |> Enum.at(0) |> String.to_integer()
      operands = split |> Enum.at(1) |> String.split(" ") |> Enum.map(&String.to_integer/1)
      %Equation{target: target, operands: operands}
    end)
    |> Enum.to_list()
  end
end
