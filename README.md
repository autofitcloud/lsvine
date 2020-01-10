# lsvine

`tree -L 2` with less empty screen space.


## Motivation

I built `lsvine` to be like [tree](https://en.wikipedia.org/wiki/Tree_(command)) but with the first-level directories distributed horizontally (and dangling downwards, hence like a vine).

This format compacts the information vertically and displays it in a [trello](https://trello.com/)-like format, one "card" per directory.

Screenshots

![](screenshots/sideBySide.png?raw=true)


## Installation

With [cargo](https://doc.rust-lang.org/cargo/)

```
cargo install lsvine
```

Downloadable binary for 64-bit linux

```
LSVINE_VERSION=0.2.5
wget https://github.com/autofitcloud/lsvine/releases/download/$LSVINE_VERSION/lsvine-v$LSVINE_VERSION-x86_64-unknown-linux-musl.tar.gz
tar -xzf lsvine-v$LSVINE_VERSION-x86_64-unknown-linux-musl.tar.gz
mv lsvine ~/.local/bin/
```


## Usage

```
# lsvine --version
lsvine 0.2.4

# lsvine .
+---------------+------------------------------------------------+-------------+---------+---------------------------+---------+
| .             | dist                                           | screenshots | src     | target                    | testdir |
+---------------+------------------------------------------------+-------------+---------+---------------------------+---------+
| CHANGELOG     | lsvine-v0.2.1-x86_64-unknown-linux-musl.tar.gz | ls.png      | main.rs | release                   | test1   |
| Cargo.lock    |                                                | lsvine.png  |         | x86_64-unknown-linux-musl | test2   |
| Cargo.toml    |                                                | tree.png    |         |                           | test3   |
| DEVELOPER.md  |                                                |             |         |                           |         |
| LICENSE       |                                                |             |         |                           |         |
| README.md     |                                                |             |         |                           |         |
| build.sh      |                                                |             |         |                           |         |
| mk_testdir.sh |                                                |             |         |                           |         |
+---------------+------------------------------------------------+-------------+---------+---------------------------+---------+
```


## The future

At some point, might want to get merged into other popular rust-based modern ls alternatives.
It could be implemented as a separate option, eg `exa --vine` or `lsd --vine`. Example repos

- [exa](https://github.com/ogham/exa)
    - (pro) It already has a [long grid view](https://the.exa.website/features/long-view#long-grid)
    - (con) Author seems too busy to dequeue issues and PRs
    - ~~(con) README doesn't list download binary from releases and run~~
        - website https://the.exa.website/ lists download binary
- [lsd](https://github.com/Peltoche/lsd)
    - (pro) Distributed via snap in addition to other channels that exa uses
    - (con) Requires some fonts as pre-requisite
- Others at [github topic = ls](https://github.com/topics/ls)



## License

Apache License 2.0. Check file [LICENSE](LICENSE)



## Dev notes

Check [DEVELOPER.md](DEVELOPER.md)
