# lsvine

List contents of directories in a vine-like format.

I built `lsvine` to be like [tree](https://en.wikipedia.org/wiki/Tree_(command)) but with the first-level directories distributed horizontally (and dangling downwards, hence like a vine).

This format compacts the information vertically and displays it in a [trello](https://trello.com/)-like format.


## Installation

Downloadable binary for 64-bit linux

```
LSVINE_VERSION=0.2.4
wget https://github.com/autofitcloud/lsvine/releases/download/$LSVINE_VERSION/lsvine-v$LSVINE_VERSION-x86_64-unknown-linux-musl.tar.gz
tar -xzf lsvine-v$LSVINE_VERSION-x86_64-unknown-linux-musl.tar.gz
mv lsvine ~/.local/bin/
```

With cargo

```
cargo install lsvine
```


## Usage

```
# lsvine --version
lsvine 0.2.2

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


## Comparison with `ls` and `tree`

- `ls` is available by default on all linux distributions.
- `tree` can be installed with `apt-get install tree`

Screenshots

![](screenshots/sideBySide.png?raw=true)

Textshots

```
# ls testdir/test1/
d1  d2  d3  f1  f2  f3

# tree testdir/test1/
testdir/
├── d1
│   ├── f4
│   ├── f5
│   └── f6
├── d2
│   ├── f7
│   ├── f8
│   └── f9
├── d3
│   ├── d4
│   │   └── f15
│   ├── f10
│   ├── f11
│   ├── f12
│   ├── f13
│   └── f14
├── f1
├── f2
└── f3

4 directories, 15 files


# lsvine testdir/test1/
+----+----+----+-----+
| .  | d1 | d2 | d3  |
+----+----+----+-----+
| f1 | f4 | f7 | d4  |
| f2 | f5 | f8 | f10 |
| f3 | f6 | f9 | f11 |
|    |    |    | f12 |
|    |    |    | f13 |
|    |    |    | f14 |
+----+----+----+-----+
```


## License

Apache License 2.0. Check file [LICENSE](LICENSE)



## Dev notes

Check [DEVELOPER.md](DEVELOPER.md)
