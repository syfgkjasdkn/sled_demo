defmodule Sled.NIF do
  @moduledoc false
  use Rustler, otp_app: :sled, crate: :sled_nif

  @type tree :: reference

  @spec open(String.t()) :: {:ok, tree} | {:error, reason :: any}
  def open(_path), do: error()

  @spec set(tree, String.t(), String.t()) :: :ok
  def set(_tree, _key, _value), do: error()

  @spec cas(tree, String.t(), String.t(), String.t()) :: :ok
  def cas(_tree, _key, _v1, _v2), do: error()

  @spec del(tree, String.t()) :: :ok
  def del(_tree, _key), do: error()

  @spec get(tree, String.t()) :: String.t() | nil
  def get(_tree, _key), do: error()

  defp error do
    :erlang.nif_error(:nif_not_loaded)
  end
end
