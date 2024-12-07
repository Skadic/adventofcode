defmodule Equation do
  defstruct [:target, :operands]

  @type t :: %__MODULE__{
          target: non_neg_integer(),
          operands: list(non_neg_integer())
        }
end
