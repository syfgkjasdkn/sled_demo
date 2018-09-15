defmodule SledTest do
  use ExUnit.Case
  doctest Sled

  test "greets the world" do
    assert Sled.hello() == :world
  end
end
