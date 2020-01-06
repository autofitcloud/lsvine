# vine

List contents of directories in a vine-like format.

Inspired by [tree](https://en.wikipedia.org/wiki/Tree_(command)) and [trello](https://trello.com)


## Installation

```
pip3 install vine
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

# vine testdir/
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

![](screenshots/vine.png?raw=true)



## Dev notes

Create the `testdir` folder

```
bash mk_testdir.sh
```

Install editably locally

```
pew new vine
pip3 install -e .
```


## License

Apache License 2.0. Check file [LICENSE](LICENSE)

