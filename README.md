# kemono-pinger

![rust](https://img.shields.io/badge/rust-2024-orange?logo=rust)
![license](https://img.shields.io/badge/license-MIT-blue)
![status](https://img.shields.io/badge/status-active-green)

a lightweight rust-based monitoring bot that watches kemono.cr creator profiles and sends discord notifications when updates are detected.

## features

- **automated polling** - continuously monitors kemono.cr creator profiles at configurable intervals
- **intelligent detection** - distinguishes between service updates, reindexing, and other changes
- **discord integration** - sends color-coded webhook notifications with rich embeds
- **gzip support** - handles compressed api responses efficiently
- **structured logging** - configurable verbosity levels for debugging
- **nix support** - reproducible builds with flake.nix

## prerequisites

- rust toolchain (edition 2024)
- a discord webhook url
- kemono.cr service and creator id to monitor

## installation

### using cargo

```bash
git clone https://github.com/yourusername/kemono-pinger.git
cd kemono-pinger
cargo build --release
```

### using nix flakes

```bash
nix build
```

## configuration

create a `config.toml` file based on `config.default.toml`:

```toml
[settings.webhook]
url = "https://discord.com/api/webhooks/YOUR_WEBHOOK_URL"

[settings.requester]
service = "patreon"           # or "fanbox", "fantia", etc.
creator_id = "123456"         # creator's numeric id
delay_ms = 60000              # polling interval in milliseconds (60s)
```

### configuration parameters

| parameter | description | example |
|-----------|-------------|---------|
| `webhook.url` | discord webhook url for notifications | `https://discord.com/api/webhooks/...` |
| `requester.service` | kemono service name | `patreon`, `fanbox`, `fantia` |
| `requester.creator_id` | creator's unique identifier | `123456` |
| `requester.delay_ms` | polling interval in milliseconds | `60000` (1 minute) |

## usage

run the bot with default settings (looks for a config.toml in current directory):

```bash
kemono-pinger
```

### command-line options

```bash
Usage: kemono-pinger [OPTIONS]

Options:
  -c, --config <FILE>          Path to the configuration file [default: config.toml]
  -v, --verbosity <VERBOSITY>  Sets the logger's verbosity level [default: INFO]
  -h, --help                   Print help
```

### examples

run with custom config:
```bash
kemono-pinger -c /path/to/config.toml
```

enable debug logging:
```bash
kemono-pinger -v debug
```

## architecture

```
src/
├── bot/
│   ├── manager.rs     # orchestrates polling loop and components
│   ├── requester.rs   # handles kemono.cr api requests
│   ├── notifier.rs    # sends discord webhook notifications
│   └── mod.rs         # api response types
└── utils/
    ├── cli.rs         # command-line argument parsing
    ├── config.rs      # configuration management
    ├── log.rs         # logging setup
    └── gunzip.rs      # gzip decompression
```

### component overview

- **manager** - main event loop that coordinates polling and notifications
- **requester** - queries kemono.cr api, detects changes, handles gzip responses
- **notifier** - formats and sends discord embeds for different event types

## notification types

the bot sends color-coded discord embeds for different events:

| event type | color | description |
|------------|-------|-------------|
| **service updated** | 🟢 green | creator posted new content |
| **service reindexed** | 🟢 green | kemono reindexed the creator's profile |
| **unknown update** | 🟢 green | detected change but type unclear |
| **error** | 🔴 red | request or parsing error occurred |

## dependencies

| crate | purpose |
|-------|---------|
| `tokio` | async runtime |
| `serenity` | discord webhook client |
| `reqwest` | http client |
| `serde`/`serde_json`/`toml` | json and toml serialization |
| `chrono` | datetime handling |
| `clap` | cli argument parsing |
| `log`/`env_logger`/`colog` | logging infrastructure |
| `flate2` | gzip decompression |

## nix integration

this project includes a nix flake for reproducible builds:

```bash
# build the project
nix build

# enter development shell
nix develop

# run directly
nix run
```

## contributing

contributions welcome! please:

1. fork the repository
2. create a feature branch
3. make your changes
4. run `cargo fmt` and `cargo clippy`
5. submit a pull request

## license

this project is licensed under the MIT license - see the [LICENSE](LICENSE) file for details.
