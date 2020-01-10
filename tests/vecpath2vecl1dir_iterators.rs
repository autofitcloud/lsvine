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
#[path = "../src/vecpath2vecl1dir_iterators.rs"]
pub mod vecpath2vecl1dir_iterators;

use vecpath2vecl1dir_iterators::{PathBufWrap, RDAdapter1, RDAdapter2};
use level1dir::Level1Dir;


// utility function for tests
pub fn _create_vecpath_twofiles_onedironefile(dir_1: &tempfile::TempDir) -> Result<Vec<std::path::PathBuf>, io::Error> {
    // a dir with 2 files

    let file_path_1 = dir_1.path().join("my-temporary-note.txt");
    File::create(&file_path_1)?;
    let file_path_2 = dir_1.path().join(".hidden.txt");
    File::create(&file_path_2)?;
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
fn transform_readdir() -> io::Result<()> {
    // Note about "?" below: failures to create tempdir/files are beyond the scope of this package, so it's ok
    let dir_1 = tempfile::tempdir()?;
    _create_vecpath_twofiles_onedironefile(&dir_1)?;
    let fs_readdir = std::fs::read_dir(dir_1.path())?;

    // get iterator
    let rda1 = vecpath2vecl1dir_iterators::transform_readdir(fs_readdir);

//           // quietly skip None values, like pandas skipna
//           .filter(|&pbw| pbw.file_name.is_some())

//    // if no files/dirs in the resultant collection
//    if level1_paths.is_empty() {
//      return std::iter::empty::<PathBufWrap>();
//    }


    // let rda1_coll: Iterator<PathBufWrap>::collect = rda1_iter.collect();
    assert_eq!(rda1.count(), 3);

    // sort because read_dir doesn't guarantee sorted order
    // rda1.sort_by_key(|pbw| pbw.path_buf);

    Ok(())
}


#[test]
fn rdadapter1_direct() -> io::Result<()> {
    // Note about "?" below: failures to create tempdir/files are beyond the scope of this package, so it's ok
    let dir_1 = tempfile::tempdir()?;
    _create_vecpath_twofiles_onedironefile(&dir_1)?;
    let fs_readdir = std::fs::read_dir(dir_1.path())?;

    // Get iterator
    // Need to consume it ONCE otherwise will get error[E0382]: use of moved value: `rda1_all`
    // let rda1_all: RDAdapter1 = RDAdapter1::new(fs_readdir).collect();
    let rda1_iter = RDAdapter1::new(fs_readdir);

    assert_eq!(rda1_iter.count(), 3);

    Ok(())
}


/// run read_dir and RDAdapter1 again on RDAdapter1's first directory
#[test]
fn rdadapter1_recur() -> io::Result<()> {
    // Note about "?" below: failures to create tempdir/files are beyond the scope of this package, so it's ok
    let dir_1 = tempfile::tempdir()?;
    _create_vecpath_twofiles_onedironefile(&dir_1)?;
    let fs_readdir = std::fs::read_dir(dir_1.path())?;

    // Get iterator then split file versus dir
    // Cannot consume more than ONCE otherwise will get error[E0382]: use of moved value: `rda1_all`
    // let rda1_all: RDAdapter1 = RDAdapter1::new(fs_readdir).collect();
    let rda1_iter = RDAdapter1::new(fs_readdir);
    let mut rda1_file: Vec<PathBufWrap> = Vec::new();
    let mut rda1_dir : Vec<PathBufWrap> = Vec::new();
    for x in rda1_iter {
      if x.path_buf.is_file() {
        rda1_file.push(x);
        continue;
      }
      // after RDAdapter1, we already don't have anything that is not file and not dir (i.e. doesnt exist)
      rda1_dir.push(x);
    }

    // assert_eq!(rda1_iter.count(), 3);
    assert_eq!(rda1_file.len() + rda1_dir.len(), 3);
    assert_eq!(rda1_file.len(), 2);
    assert_eq!(rda1_dir.len(), 1);

    // Iterate over each dir and use RDAdapter1 again
    // No need to filter for non-nulls (not sure why)
    for l1dir in &rda1_dir {
      // read_dir again, but this time we care about errors
      match std::fs::read_dir(&l1dir.path_buf) {
        Ok(fs_readdir_2) => {
          // get iterator
          let rda2_all = RDAdapter1::new(fs_readdir_2);
          assert_eq!(rda2_all.count(), 1);
        },
        Err(_e) => {
          // failed to read directory contents
          assert_eq!(true, false);
        }
      }
    }

    Ok(())
}



#[test]
fn rdadapter2() -> io::Result<()> {
    // Note about "?" below: failures to create tempdir/files are beyond the scope of this package, so it's ok
    let dir_1 = tempfile::tempdir()?;
    _create_vecpath_twofiles_onedironefile(&dir_1)?;

    // Get iterator
    // Need to consume it ONCE otherwise will get error[E0382]: use of moved value: `rda1_all`
    // let rda1_all: RDAdapter1 = RDAdapter1::new(fs_readdir).collect();
    let rda2_iter = RDAdapter2::new(dir_1.path());
    //let rda2_coll = rda2_iter.collect::<Vec<Vec<PathBufWrap>>>();
    let rda2_coll = rda2_iter.collect::<Vec<Level1Dir>>();

    // 2 files + 1 dir
    assert_eq!(rda2_coll.len(), 2);
    assert_eq!(rda2_coll[0].contents.len(), 2);
    assert_eq!(rda2_coll[1].contents.len(), 1);

    Ok(())
}
