# shwebhook
shwebhook is a simple Rust CLI tool to send a Discord webhook message in, for example, a shell script.

## Configuration
shwebhook will try to find a file named `shwebhook.toml` in its CWD. If it can't find it,
it will fall back to reading `/etc/shwebhook.toml`.

Configuration is done in this TOML file.

An example configuration:
```toml
default_url = "https://canary.discord.com/webhooks/1234567890/TOKEN.HERE/"
default_profile = "basic"

[[profiles]]
name = "basic"
username = "Your Username" # optional
avatar = "Your Avatar" # optional
url = "https://canary.discord.com/webhooks/1234567890/TOKEN.HERE/" # optional, overrides default_url
```

Each config file consists of a default URL, a default profile, and one or more profiles.
A profile is composed of a name, a username (optional), an avatar (optional), and a URL override (optional).

## Usage
To send a message to the default profile, use `shwebhook -- "your message goes here"`.
To specify a profile explicitly, use the `-p` or `--profile` flag.
To specify a username explicitly, use the `-u` or `--username` flag.