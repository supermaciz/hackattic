# HelpMeUnpack (Elixir)

Small Elixir client for the Hackattic "Help me unpack" challenge.

It:
- fetches the base64-encoded byte payload from Hackattic
- unpacks the values using bitstring pattern matching (little-endian + one big-endian double)
- submits the resulting JSON back to Hackattic

## Requirements

- Elixir ~> 1.19

## Run

1) Fetch deps:

```sh
mix deps.get
```

2) Set your Hackattic access token (used by `config/runtime.exs`):

```sh
export HACKATTIC_TOKEN="YOUR_TOKEN"
```

3) Run:

```sh
mix run -e 'HelpMeUnpack.solve()'
```

If you prefer IEx:

```sh
HACKATTIC_TOKEN="YOUR_TOKEN" iex -S mix
```

```elixir
HelpMeUnpack.solve()
```
