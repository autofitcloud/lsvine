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
#[path = "../src/vecpath2vecl1dir_onefunc.rs"]
pub mod vecpath2vecl1dir_onefunc;

pub use vecpath2vecl1dir_onefunc::{vecpath2vecl1dir, _create_vecpath_twofiles_onedironefile};


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




#[test]
fn test_vecpath2vecl1dir_twofiles_onedironefile() -> io::Result<()> {
    let dir_1 = tempfile::tempdir()?;
    let input = _create_vecpath_twofiles_onedironefile(&dir_1)?;
    let actual = vecpath2vecl1dir(input)?;
    assert_eq!(actual.len(), 2);
    assert_eq!(actual[0].contents.len(), 2);

    Ok(())
}

