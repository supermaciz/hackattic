import Config

config :help_me_unpack, HelpMeUnpack.HackatticAPI,
  access_token: System.fetch_env!("HACKATTIC_TOKEN")
