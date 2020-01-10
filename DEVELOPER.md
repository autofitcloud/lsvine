## Dev notes

### Testing

Create the `testdir` folder

```
bash mk_testdir.sh
```

Test

```
lsvine testdir/test1/ # standard use case
lsvine testdir/test2/ # empty
lsvine testdir/test3/ # 30 dirs with nested 30 dirs
```

### Moving from python to rust

Python code still available in git repo branch "python".

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
- https://blog.cryptowat.ch/2019/11/25/sponsoring-rust-gui-library-iced/
  - memory footprint


Following tutorial

- https://www.rust-lang.org/what/cli
- https://rust-cli.github.io/book/tutorial/setup.html
- https://www.rust-lang.org/tools/install


### Build & run

```
# seq 1 20; cargo run testdir/test3
```


### Build and upload release to github

Check tests pass

```
cargo test
```

Lint with [clippy](https://github.com/rust-lang/rust-clippy)

```
cargo clippy
```

Update versions in

- CHANGELOG
- Cargo.toml
- README.md
- build.sh

Re-run `cargo build` to update Cargo.lock

Commit version bump and push

Re-run `cargo test` out of anxiety

git tag and push

Build release

```
bash build.sh
```

Upload `dist/lsvine*` to github.com/autofitcloud/lsvine after creating a new release.

Release to crates.io with `cargo publish`


### TODO

TODO mac?

TODO distribute via snap et al:

Build a snap? (for ubuntu, debian, fedora, archlinux, rasperry pi, linux mint, ...)
- https://snapcraft.io/blog/building-a-rust-snap-by-example
- https://newspuddle.com/building-a-rust-snap-by-example/
- "measure user growth" :)

alternatives:
- https://appimage.org/
- https://www.reddit.com/r/rust/comments/6o57em/distributing_binaries_through_ppas/
