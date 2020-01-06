# lsvine

List contents of directories in a vine-like format.

`lsvine` is like [tree](https://en.wikipedia.org/wiki/Tree_(command)) but with the first-level directories distributed horizontally (and dangling downwards, hence like a vine).


## Installation

```
pip3 install lsvine
```

## Usage

```
# lsvine --version
lsvine, version 0.1.0

# lsvine .
screenshots    testdir    lsvine       lsvine.egg-info       dot
-------------  ---------  -----------  --------------------  -------------
ls.png         d1         __init__.py  PKG-INFO              CHANGELOG
tree.png       d2         __pycache__  SOURCES.txt           DEVELOPER.md
lsvine.png     d3         cli.py       dependency_links.txt  LICENSE
               f1                      entry_points.txt      README.md
               f2                      requires.txt          mk_testdir.sh
               f3                      top_level.txt         setup.py

```


## Comparison with `ls` and `tree`

- `ls` is available by default on all linux distributions.
- `tree` can be installed with `apt-get install tree`

```
# ls testdir/
d1  d2  d3  f1  f2  f3

# tree testdir/
testdir/
├── d1
│   ├── f4
│   ├── f5
│   ├── f6
│   └── f7
├── d2
│   ├── f4
│   ├── f5
│   └── f6
├── d3
│   ├── d4
│   │   └── f15
│   ├── f10
│   ├── f11
│   ├── f12
│   ├── f13
│   ├── f14
│   ├── f4
│   ├── f5
│   └── f6
├── f1
├── f2
└── f3

4 directories, 19 files

# lsvine testdir/
d1    d2    d3    dot
----  ----  ----  -----
f4    f4    d4    f1
f5    f5    f10   f2
f6    f6    f11   f3
f7          f12
            f13
            f14
            f4
            f5
            f6

```

Screenshots

![](screenshots/ls.png?raw=true)

![](screenshots/tree.png?raw=true)

![](screenshots/lsvine.png?raw=true)



## License

Apache License 2.0. Check file [LICENSE](LICENSE)



## Dev notes

Check [DEVELOPER.md](DEVELOPER.md)
