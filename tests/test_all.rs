// tests https://rust-cli.github.io/book/tutorial/testing.html

use std::io;

// for tempfiles
use std::fs::{File, create_dir};

// imports
// In ./tests it's a bit different than in ./src
// Check https://github.com/rust-lang/rust-clippy/blob/2e8c3c3e9eb81bc2be047b1d4b2f9f31b869b6f5/tests/ui/crashes/ice-4671.rs#L5
#[path = "../src/tablebuf.rs"]
pub mod tablebuf;
#[path = "../src/level1dir.rs"]
pub mod level1dir;
#[path = "../src/vecpath2vecl1dir.rs"]
pub mod vecpath2vecl1dir;

pub use tablebuf::TableBuf;
pub use level1dir::Level1Dir;
pub use vecpath2vecl1dir::vecpath2vecl1dir;


#[test]
fn test_vecpath2vecl1dir_emptyvec() -> io::Result<()> {
    // empty
    let input = Vec::new();
    let actual = vecpath2vecl1dir(input)?;
    assert_eq!(actual.len(), 0);

    Ok(())
}

#[test]
fn test_vecpath2vecl1dir_onefile() -> io::Result<()> {
    // Use tempfile and tempdir
    // https://doc.rust-lang.org/std/path/struct.PathBuf.html#examples
    // https://doc.rust-lang.org/std/env/fn.temp_dir.html
    // https://github.com/Stebalien/tempfile

    // single file, not dir
    let tmpfile = tempfile::NamedTempFile::new()?;
    let mut input = Vec::new();
    input.push(tmpfile.path().to_path_buf());

    let actual = vecpath2vecl1dir(input)?;
    assert_eq!(actual.len(), 1);

    Ok(())
}

#[test]
fn test_vecpath2vecl1dir_twofiles_only() -> io::Result<()> {
    // a dir with 2 files
    let dir = tempfile::tempdir()?;
    let file_path_1 = dir.path().join("my-temporary-note.txt");
    File::create(&file_path_1)?;
    let file_path_2 = dir.path().join("another-note.txt");
    File::create(&file_path_2)?;
    let mut input = Vec::new();
    input.push(file_path_1.to_path_buf());
    input.push(file_path_2.to_path_buf());

    let actual = vecpath2vecl1dir(input)?;
    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0].contents.len(), 2);

    Ok(())
}

#[test]
fn test_vecpath2vecl1dir_twofiles_onedirempty() -> io::Result<()> {
    // a dir with 2 files
    let dir_1 = tempfile::tempdir()?;

    let file_path_1 = dir_1.path().join("my-temporary-note.txt");
    File::create(&file_path_1)?;
    let file_path_2 = dir_1.path().join("another-note.txt");
    File::create(&file_path_2)?;
    let dir_path_2 = dir_1.path().join("subdir");
    create_dir(&dir_path_2)?;

    let mut input = Vec::new();
    input.push(file_path_1.to_path_buf());
    input.push(file_path_2.to_path_buf());
    input.push(dir_path_2.to_path_buf());

    let actual = vecpath2vecl1dir(input)?;
    assert_eq!(actual.len(), 2);
    assert_eq!(actual[0].contents.len(), 2);
    assert_eq!(actual[1].contents.len(), 0);

    Ok(())
}


// utility function for tests
fn _create_vecpath_twofiles_onedironefile(dir_1: &tempfile::TempDir) -> Result<Vec<std::path::PathBuf>, io::Error> {
    // a dir with 2 files

    let file_path_1 = dir_1.path().join("my-temporary-note.txt");
    File::create(&file_path_1)?;
    let file_path_2 = dir_1.path().join("another-note.txt");
    File::create(&file_path_2)?;
    let dir_path_2 = dir_1.path().join("subdir");
    create_dir(&dir_path_2)?;
    let file_path_3 = dir_path_2.join("some-pic.txt");
    File::create(&file_path_3)?;

    let mut input = Vec::new();
    input.push(file_path_1);
    input.push(file_path_2);
    input.push(dir_path_2);
    // no need to insert this, the function will traverse again and find it
    // input.push(file_path_3.to_path_buf());

    Ok(input)
}


#[test]
fn test_vecpath2vecl1dir_twofiles_onedironefile() -> io::Result<()> {
    let dir_1 = tempfile::tempdir()?;
    let input = _create_vecpath_twofiles_onedironefile(&dir_1)?;
    let actual = vecpath2vecl1dir(input)?;
    assert_eq!(actual.len(), 2);
    assert_eq!(actual[0].contents.len(), 2);

    Ok(())
}


#[test]
fn test_tablebuf() -> io::Result<()> {
    let _terminal_width = 100;
    let n_l1dirs = 5;
    let mut level1_vine = TableBuf::new(_terminal_width, n_l1dirs);

    // on start, no need to flush
    assert_eq!(level1_vine.table.len(), 0);
    assert_eq!(level1_vine.should_flush(), false);

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
    level1_vine.push(&l1dir_1);

    // should now have 2 columns and 2 rows
    assert_eq!(level1_vine.table.len(), 3);
    assert_eq!(level1_vine.table[0].len(), 1);
    assert_eq!(level1_vine.table[1].len(), 1);
    assert_eq!(level1_vine.table[2].len(), 1);

    // still, no need to flush
    assert_eq!(level1_vine.should_flush(), false);

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
    level1_vine.push(&l1dir_2);

    // max_name_len > terminal_width => should_flush = true
    assert_eq!(level1_vine.should_flush(), true);

    Ok(())
}

