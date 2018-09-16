defmodule SledTest do
  use ExUnit.Case

  setup do
    {:ok, tree} = Sled.open("/tmp/sled")

    # on_exit(fn -> File.rm!("/tmp/sled") end)
    :ok = Sled.del(tree, "hello")

    {:ok, tree: tree}
  end

  test "it works", %{tree: tree} do
    assert Sled.get(tree, "hello") == nil
    assert Sled.set(tree, "hello", "world") == :ok
    assert Sled.get(tree, "hello") == 'world'
  end
end
