# Configr

The dead easy way to use config files in your project\
\
[![crates.io](https://img.shields.io/crates/v/configr.svg)](https://crates.io/crates/configr)

This will load a `config.toml` file if it exists, otherwise it will
create the needed folders and the toml file.\
It can either use the OS config directories which are as follows

- Linux: `$XDG_CONFIG_HOME/app-name/config.toml`
- Windows: `%APPDATA%/app-name/config.toml`
- Mac OS: `$HOME/Library/Application Support/app-name/config.toml`

or a custom config directory

## Usage

Add the following to your `Cargo.toml`

```toml
configr = "0.7.0"
```

or use [cargo-edit](https://github.com/killercup/cargo-edit/) with
`cargo add configr`

then in your project add the following snippet

```rust
use configr::{Config, ConfigrDefault};
#[derive(ConfigrDefault, Deserialize, Serialize, Default)]
pub struct BotConfig {
    bot_username: String,
    client_id: String,
    client_secret: String,
    channel: String,
}
```

replacing BotConfig with your configuration struct

and then load you can load the config, usually at the start of the
application with the `load` function to load from the system config
directory

```rust
let config = BotConfig::load("bot app", true).unwrap(); // Will load from /home/USER/.config/bot-app/config.toml
```

or with the `load_with_dir` function to use a custom config
directory

```rust
let config = BotConfig::load_with_dir("bot app", "$HOME").unwrap(); // Will load from /home/USER/bot-app/config.toml
```

Use `ConfigrDefault` if you want the config.toml to be populated with
default values, based on Default implentation, or use `Configr` if
you want it to be populated with empty fields with the correct names

Example of above BotConfig\
With `Configr`

```toml
bot_username=
client_id=
client_secret=
channel=
```

and with `ConfigrDefault`

```toml
bot_username=""
client_id=""
client_secret=""
channel=""
```

## Contributors

I am at the moment not accepting any contributions that don't close an issue.\
If you find any problems, or edge cases, please do open an issue!

## License

This project is licensed under the [unlicense](https://unlicense.org/) license.
