defmodule Sled do
  @moduledoc """
  Documentation for Sled.
  """

  defdelegate open(path), to: __MODULE__.NIF
  defdelegate set(tree, key, value), to: __MODULE__.NIF
  defdelegate cas(tree, key, v1, v2), to: __MODULE__.NIF
  defdelegate del(tree, key), to: __MODULE__.NIF
  defdelegate get(tree, key), to: __MODULE__.NIF
end
