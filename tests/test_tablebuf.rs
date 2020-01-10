// tests https://rust-cli.github.io/book/tutorial/testing.html

use std::io;


// imports
// In ./tests it's a bit different than in ./src
// Check https://github.com/rust-lang/rust-clippy/blob/2e8c3c3e9eb81bc2be047b1d4b2f9f31b869b6f5/tests/ui/crashes/ice-4671.rs#L5
#[path = "../src/tablebuf.rs"]
pub mod tablebuf;
#[path = "../src/level1dir.rs"]
pub mod level1dir;
#[path = "../src/vecpath2vecl1dir_onefunc.rs"]
pub mod vecpath2vecl1dir_onefunc;
pub use vecpath2vecl1dir_onefunc::_create_vecpath_twofiles_onedironefile;

pub use tablebuf::TableBuf;
pub use level1dir::Level1Dir;


#[test]
fn test_tablebuf() -> io::Result<()> {
    let _terminal_width = 100;
    let n_l1dirs = 5;
    let mut level1_vine = TableBuf::new(_terminal_width, n_l1dirs);

    // on start, no need to flush
    assert_eq!(level1_vine.table.len(), 0);

    // display doesn't do anything
    level1_vine.display();
    level1_vine.flush();
    assert_eq!(level1_vine.table.len(), 0);

    // create a Level1Dir object for testing push/display/flush/should_flush
    let dir_1 = tempfile::tempdir()?;
    let l1dir_1 = Level1Dir {
      dirname: String::from("whatever"),
      contents: _create_vecpath_twofiles_onedironefile(&dir_1)?,
      max_name_len: 20
    };

    // still, no need to flush
    assert_eq!(level1_vine.should_flush(&l1dir_1), false);

    // push
    level1_vine.push(&l1dir_1);

    // should now have 2 columns and 2 rows
    assert_eq!(level1_vine.table.len(), 3);
    assert_eq!(level1_vine.table[0].len(), 1);
    assert_eq!(level1_vine.table[1].len(), 1);
    assert_eq!(level1_vine.table[2].len(), 1);

    // display/flush does stuff, but we don't care ATM as long as there are no errors
    level1_vine.display();
    level1_vine.flush();
    assert_eq!(level1_vine.table.len(), 0);

    // another l1dir with a longer name
    let dir_2 = tempfile::tempdir()?;
    let l1dir_2 = Level1Dir {
      dirname: String::from("whatever"),
      contents: _create_vecpath_twofiles_onedironefile(&dir_2)?,
      max_name_len: 200
    };

    // max_name_len > terminal_width => should_flush = true
    assert_eq!(level1_vine.should_flush(&l1dir_2), true);

    Ok(())
}

