defmodule Cubes do
  defstruct red: 0, green: 0, blue: 0

  @spec from_map(any()) :: %Cubes{}
  def from_map(map) do
    struct(Cubes, map)
  end

  @spec max_col(%Cubes{}, %Cubes{}) :: %Cubes{}
  def max_col(c1, c2) do
    %Cubes{red: max(c1.red, c2.red), green: max(c1.green, c2.green), blue: max(c1.blue, c2.blue)}
  end

  @spec power(%Cubes{}) :: integer()
  def power(color) do
    color.red * color.green * color.blue
  end

  @spec is_possible(%Cubes{}, %Cubes{}) :: boolean()
  def is_possible(cubes, restriction) do
    cubes.red <= restriction.red && cubes.green <= restriction.green &&
      cubes.blue <= restriction.blue
  end
end
