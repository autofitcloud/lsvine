# lsvine

`tree -L 2` with less empty screen space.

`ls *` with vertical filename listing.


## Motivation

I built `lsvine` to be like [tree](https://en.wikipedia.org/wiki/Tree_(command)) but with the first-level directories distributed horizontally (and dangling downwards, hence like a vine).

This format compacts the information vertically and displays it in a [trello](https://trello.com/)-like format, one "card" per directory.

Screenshots

<!-- ![](screenshots/sideBySide.png?raw=true) -->

<!-- edit the image on imgur at https://imgur.com/a/CvTgoR7 in my shadi@autofitcloud.com account -->
![Imgur Image](http://i.imgur.com/5QYqemF.png)


Comparison to `ls *`

```
# ls *
cli.py  corpus.py  corpus_ytq1_v20210524e.py  datasets.py  ffmpeg.py  __init__.py  s3_archive_cli.py  s3_tarfile.py  s3_zipfile.py  spectrogram.py  test_bla.py  test_utils.py  utils.py

io:
audiofile.py  dir_tgz.py  gallery_html_generator.py  gs_dir_struct_mirror.py  __init__.py

pipeline:
base_multiple.py  __init__.py  local_multiple.py  local_single.py  remote_gs.py  remote_youtube.py

spectrogram_helpers:
__init__.py                     s14_drop_periodic_blobs.py   s22a_watershed.py          s23b_whistle_segwrap.py  s42_tracer.py          test_s22a_watershed.py
s12_drop_bursts.py              s21a_drop_blobs.py           s22b_whistle_connector.py  s3_reporter.py           s43_transliterator.py  test_s43_transliterator.py
s13_remove_noise__blurflood.py  s21b_strengthen_whistles.py  s23a_whistle_segmenter.py  s41_stats.py             s44_subtitles.py
```

```
# lsvine
+---------------------------+---------------------------+-------------------+--------------------------------+
| .                         | io                        | pipeline          | spectrogram_helpers            |
+---------------------------+---------------------------+-------------------+--------------------------------+
| __init__.py               | __init__.py               | __init__.py       | __init__.py                    |
| cli.py                    | audiofile.py              | base_multiple.py  | s12_drop_bursts.py             |
| corpus.py                 | dir_tgz.py                | local_multiple.py | s13_remove_noise__blurflood.py |
| corpus_ytq1_v20210524e.py | gallery_html_generator.py | local_single.py   | s14_drop_periodic_blobs.py     |
| datasets.py               | gs_dir_struct_mirror.py   | remote_gs.py      | s21a_drop_blobs.py             |
| ffmpeg.py                 |                           | remote_youtube.py | s21b_strengthen_whistles.py    |
| s3_archive_cli.py         |                           |                   | s22a_watershed.py              |
| s3_tarfile.py             |                           |                   | s22b_whistle_connector.py      |
| s3_zipfile.py             |                           |                   | s23a_whistle_segmenter.py      |
| spectrogram.py            |                           |                   | s23b_whistle_segwrap.py        |
| test_bla.py               |                           |                   | s3_reporter.py                 |
| test_utils.py             |                           |                   | s41_stats.py                   |
| utils.py                  |                           |                   | s42_tracer.py                  |
|                           |                           |                   | s43_transliterator.py          |
|                           |                           |                   | s44_subtitles.py               |
|                           |                           |                   | test_s22a_watershed.py         |
|                           |                           |                   | test_s43_transliterator.py     |
+---------------------------+---------------------------+-------------------+--------------------------------+

```

## Installation

With [cargo](https://doc.rust-lang.org/cargo/)

```
cargo install lsvine
```

Downloadable binary for 64-bit linux

```
LSVINE_VERSION=0.3.1
wget https://github.com/autofitcloud/lsvine/releases/download/$LSVINE_VERSION/lsvine-v$LSVINE_VERSION-x86_64-unknown-linux-musl.tar.gz
tar -xzf lsvine-v$LSVINE_VERSION-x86_64-unknown-linux-musl.tar.gz
mv lsvine ~/.local/bin/
```


## Usage

Regular usage:

```
# lsvine --version
lsvine 0.3.1

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

Show hidden filenames

```
# lsvine -a
+----------------+----------------+------------------------------------------------+-----------------------+-------------------------------+---------------------------+----------+-------------------------------+
| .              | .git           | dist                                           | screenshots           | src                           | target                    | testdir  | tests                         |
+----------------+----------------+------------------------------------------------+-----------------------+-------------------------------+---------------------------+----------+-------------------------------+
| .README.md.swp | COMMIT_EDITMSG | .gitkeep                                       | sideBySide-latest.png | level1dir.rs                  | .gitkeep                  | .gitkeep | test_tablebuf.rs              |
| .gitignore     | FETCH_HEAD     | lsvine-v0.3.1-x86_64-unknown-linux-musl.tar.gz |                       | longest_common_prefix.rs      | .rustc_info.json          | test1    | vecpath2vecl1dir_iterators.rs |
| CHANGELOG      | HEAD           |                                                |                       | main.rs                       | package                   | test2    | vecpath2vecl1dir_onefunc.rs   |
| Cargo.lock     | ORIG_HEAD      |                                                |                       | main_bkp_onefunc.rs           | release                   | test3    |                               |
| Cargo.toml     | branches       |                                                |                       | tablebuf.rs                   | x86_64-unknown-linux-musl |          |                               |
| DEVELOPER.md   | config         |                                                |                       | vecpath2vecl1dir_iterators.rs |                           |          |                               |
| LICENSE        | description    |                                                |                       | vecpath2vecl1dir_onefunc.rs   |                           |          |                               |
| README.md      | hooks          |                                                |                       |                               |                           |          |                               |
| build.sh       | index          |                                                |                       |                               |                           |          |                               |
| mk_testdir.sh  | info           |                                                |                       |                               |                           |          |                               |
|                | logs           |                                                |                       |                               |                           |          |                               |
|                | objects        |                                                |                       |                               |                           |          |                               |
|                | refs           |                                                |                       |                               |                           |          |                               |
+----------------+----------------+------------------------------------------------+-----------------------+-------------------------------+---------------------------+----------+-------------------------------+
```

Contract filename suffixes to reduce occupied screen-space further:

```
# lsvine testdir/test1
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

# lsvine testdir/test1 --contract-suffix
+--------+--------+--------+---------+
| .      | d1     | d2     | d3      |
+--------+--------+--------+---------+
| f* (3) | f* (3) | f* (3) | d4      |
|        |        |        | f1* (5) |
+--------+--------+--------+---------+

# lsvine testdir/test1 --contract-suffix --minimum-prefix-length=2
+----+----+----+---------+
| .  | d1 | d2 | d3      |
+----+----+----+---------+
| f1 | f4 | f7 | d4      |
| f2 | f5 | f8 | f1* (5) |
| f3 | f6 | f9 |         |
+----+----+----+---------+
```

For example, `lsvine -c -m 3 /etc` output [here](https://gist.github.com/shadiakiki1986/76d7a961d835400ed1d02e2d34c867e2)


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


## Author

Built by ❤ [AutofitCloud](https://www.autofitcloud.com).

To follow announcements 📢:

- [twitter](https://twitter.com/autofitcloud)
- [reddit](https://www.reddit.com/r/autofitcloud)
- [discord](https://discord.gg/56zmfDc)
