defmodule HelpMeUnpack.HackatticAPI do
  @spec get_pack!() :: binary()
  def get_pack! do
    Req.get!("https://hackattic.com/challenges/help_me_unpack/problem",
      params: [access_token: token()]
    )
    |> case do
      %Req.Response{status: 200, body: %{"bytes" => bytes_base64}} ->
        bytes_base64

      %Req.Response{status: status, body: body} ->
        raise "HackatticAPI error. Status #{status}. #{inspect(body)}"
    end
  end

  def submit_solution(solution) do
    Req.post!("https://hackattic.com/challenges/help_me_unpack/solve",
      params: [access_token: token()],
      json: solution
    )
  end

  defp token do
    Application.get_env(:help_me_unpack, __MODULE__)[:access_token]
  end
end
