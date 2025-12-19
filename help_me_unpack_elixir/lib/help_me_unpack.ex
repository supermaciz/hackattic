defmodule HelpMeUnpack do
  alias HelpMeUnpack.HackatticAPI

  def solve do
    HackatticAPI.get_pack!()
    |> IO.inspect(label: "bytes base64")
    |> unpack!()
    |> IO.inspect(label: "My solution")
    |> HackatticAPI.submit_solution()
  end

  def unpack!(data) do
    bits = Base.decode64!(data)

    <<int::signed-integer-little-size(32), uint::unsigned-integer-little-size(32),
      short::signed-integer-little-size(32), float::float-little-size(32),
      double::float-little-size(64), big_double::float-big-size(64)>> = bits

    %{
      int: int,
      uint: uint,
      short: short,
      float: float,
      double: double,
      big_endian_double: big_double
    }
  end
end
