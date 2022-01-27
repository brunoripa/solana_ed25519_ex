defmodule SolanaEDTest do
  use ExUnit.Case
  doctest SolanaED

  test "greets the world" do
    assert SolanaED.hello() == :world
  end
end
