# bump
send `!d bump` to a Discord channel every 2 hours.

## How to run
```bash
git clone https://github.com/c8hunderscore/bump # git must be installed https://git-scm.com/downloads
cd bump
cargo run # rust must be installed https://www.rust-lang.org/tools/install
```

## How to set environment variable
Linux:
```bash
export DISCORD_TOKEN="your token here"
```
(if you want this to be set persistently, rather than manually, add it to `~/.bashrc`

Windows:
```batch
setx DISCORD_TOKEN "your token here"
```
