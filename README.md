# lsvine

List contents of directories in a vine-like format.

I built `lsvine` to be like [tree](https://en.wikipedia.org/wiki/Tree_(command)) but with the first-level directories distributed horizontally (and dangling downwards, hence like a vine).

This format compacts the information vertically and displays it in a [trello](https://trello.com/)-like format.


## Installation

```
pip3 install lsvine
```

## Usage

```
# lsvine --version
lsvine, version 0.1.0

# lsvine .
0_root         build               dist                           lsvine       lsvine.egg-info
-------------  ------------------  -----------------------------  -----------  --------------------
CHANGELOG      bdist.linux-x86_64  lsvine-0.1.2-py3-none-any.whl  __init__.py  PKG-INFO
DEVELOPER.md   lib                 lsvine-0.1.2.tar.gz            __pycache__  SOURCES.txt
LICENSE                                                           cli.py       dependency_links.txt
README.md                                                                      entry_points.txt
mk_testdir.sh                                                                  requires.txt
setup.py                                                                       top_level.txt

screenshots    testdir
-------------  ---------
ls.png         d1
lsvine.png     d2
tree.png       d3
               f1
               f2
               f3

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


# lsvine testdir/
0_root    d1    d2    d3
--------  ----  ----  ----
f1        f4    f7    d4
f2        f5    f8    f10
f3        f6    f9    f11
                      f12
                      f13
                      f14

```

Screenshots

![](screenshots/ls.png?raw=true)

![](screenshots/tree.png?raw=true)

![](screenshots/lsvine.png?raw=true)



## License

Apache License 2.0. Check file [LICENSE](LICENSE)



## Dev notes

Check [DEVELOPER.md](DEVELOPER.md)
