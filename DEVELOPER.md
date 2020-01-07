## Dev notes

Create the `testdir` folder

```
bash mk_testdir.sh
```

Create a git orphan branch

- https://gist.github.com/seanbuscay/5877413


Why move from python to rust

- https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust/
- https://www.reddit.com/r/linux/comments/eksf2f/lsvine_list_contents_of_directories_in_a_vinelike/fdfqna0?utm_source=share&utm_medium=web2x
    - speed
        - easy to observe that this is much faster than the python implementation
    - binary size
        - indeed, `du -sh target/debug/lsvine` shows 17M
        - whereas `du -sh /home/shadi/.local/share/virtualenvs/lsvine/lib/python3.6/site-packages/` shows `171M`


Following tutorial

- https://www.rust-lang.org/what/cli
- https://rust-cli.github.io/book/tutorial/setup.html
- https://www.rust-lang.org/tools/install


Build & run

```
# seq 1 20; cargo run testdir/test3
```
