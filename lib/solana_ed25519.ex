defmodule SolanaED do
  @moduledoc """
  Documentation for `SolanaEd25519`.
  """
  use Rustler, otp_app: :solanaed, crate: "solanaed"

  def generate, do: :erlang.nif_error(:nif_not_loaded)
end
