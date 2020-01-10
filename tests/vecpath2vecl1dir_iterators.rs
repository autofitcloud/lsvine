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

// File or dir, for use in create_dirstructure
// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
// https://stackoverflow.com/questions/40559931/vector-store-mixed-types-of-data-in-rust
pub enum FoD {
  F(i32),
  H(i32), // hidden file (starts with .)
  D(Vec<FoD>)
}

use std::fmt;
impl fmt::Display for FoD {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        FoD::F(n_files) => {
          write!(f, "f{}", n_files)
        },
        FoD::H(n_files) => {
          write!(f, "h{}", n_files)
        },
        FoD::D(vec_files_or_dirs) => {
          write!(f, "d{}", vec_files_or_dirs.len())
        }
      }
  }
}

// utility function for tests
// root_nfiles: number of files in the root dir
// dir_struct: list of int or list describing the structure expected
//             eg [2, [0,2]] means 2 files in root, 1 dir in root, 0 files in first dir in root, 2 dirs in first dir in root
pub fn create_dirstructure(rootdir_path: &std::path::Path, dir_struct: &Vec<FoD>, level_opt: Option<i32>, input_opt: Option<Vec<std::path::PathBuf>>) -> Result<Vec<std::path::PathBuf>, io::Error> {
    // http://stackoverflow.com/questions/24047686/ddg#35369909
    // note that this doesn't get passed into the recursion, hence only contains first-level data
    let mut input_vec = input_opt.unwrap_or(Vec::new());

    let level_int = level_opt.unwrap_or(0);

    println!("cretae_direstruct {}: enter", level_int);

    for (j, dir_content) in dir_struct.iter().enumerate() {
      // https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
      let mut prefix = "";
      if let FoD::H(_n_files) = dir_content { prefix = "."; }
      println!("level {}, dirconten {}, prefix {}", level_int, dir_content, prefix);

      match dir_content {
        FoD::F(n_files) | FoD::H(n_files) => {
          for i in 0..*n_files {
            let file_path = rootdir_path.join(format!("{}f_{}_{}_{}.txt", prefix, level_int, j, i));
            File::create(&file_path)?; // raise on error
            if prefix != "." {
              input_vec.push(file_path);
            }
          }
        },
        FoD::D(vec_files_or_dirs) => {
          let subdir_path = rootdir_path.join(format!("d_{}_{}", level_int, j));
          create_dir(&subdir_path)?; // raise on error
          input_vec.push(subdir_path);
          let _ = create_dirstructure(&input_vec.last().unwrap(), &vec_files_or_dirs, Some(level_int+1), None);
        }
      }
    }

    println!("cretae_direstruct {}: leave", level_int);

    Ok(input_vec)
}


// utility variable
fn dirstruct_f2_d1() -> Vec<FoD> { vec![FoD::F(2), FoD::H(1), FoD::D(vec![FoD::H(1), FoD::F(1)])] }

#[test]
fn test_create_dirstructure() -> io::Result<()> {
    // Need to return Result in order to benefit from "?" after tempdir() below
    let dir_1 = tempfile::tempdir()?;
    let input_vec = create_dirstructure(dir_1.path(), &dirstruct_f2_d1(), None, None);
    assert_eq!(input_vec.is_ok(), true);
    assert_eq!(input_vec.unwrap().len(), 3);
    Ok(())
}


#[test]
fn transform_readdir() -> io::Result<()> {
    // Note about "?" below: failures to create tempdir/files are beyond the scope of this package, so it's ok
    let dir_1 = tempfile::tempdir()?;
    create_dirstructure(&dir_1.path(), &dirstruct_f2_d1(), None, None)?;
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
    create_dirstructure(&dir_1.path(), &dirstruct_f2_d1(), None, None)?;

    let fs_readdir = std::fs::read_dir(dir_1.path())?;

    // Get iterator
    // Need to consume it ONCE otherwise will get error[E0382]: use of moved value: `rda1_all`
    // let rda1_all: RDAdapter1 = RDAdapter1::new(fs_readdir).collect();
    let rda1_iter = RDAdapter1::new(fs_readdir, false);

    assert_eq!(rda1_iter.count(), 3);

    Ok(())
}


/// run read_dir and RDAdapter1 again on RDAdapter1's first directory
#[test]
fn rdadapter1_recur() -> io::Result<()> {
    // Note about "?" below: failures to create tempdir/files are beyond the scope of this package, so it's ok
    let dir_1 = tempfile::tempdir()?;
    create_dirstructure(&dir_1.path(), &dirstruct_f2_d1(), None, None)?;

    let fs_readdir = std::fs::read_dir(dir_1.path())?;

    // Get iterator then split file versus dir
    // Cannot consume more than ONCE otherwise will get error[E0382]: use of moved value: `rda1_all`
    // let rda1_all: RDAdapter1 = RDAdapter1::new(fs_readdir).collect();
    let rda1_iter = RDAdapter1::new(fs_readdir, false);
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
          let rda2_all = RDAdapter1::new(fs_readdir_2, false);
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
fn rdadapter2_hidedot() -> io::Result<()> {
    // Note about "?" below: failures to create tempdir/files are beyond the scope of this package, so it's ok
    let dir_1 = tempfile::tempdir()?;
    create_dirstructure(&dir_1.path(), &dirstruct_f2_d1(), None, None)?;

    // Get iterator
    // Need to consume it ONCE otherwise will get error[E0382]: use of moved value: `rda1_all`
    // let rda1_all: RDAdapter1 = RDAdapter1::new(fs_readdir).collect();
    let rda2_iter = RDAdapter2::new(dir_1.path(), false);
    //let rda2_coll = rda2_iter.collect::<Vec<Vec<PathBufWrap>>>();
    let rda2_coll = rda2_iter.collect::<Vec<Level1Dir>>();

    // 2 files + 1 dir
    assert_eq!(rda2_coll.len(), 2);
    assert_eq!(rda2_coll[0].contents.len(), 2);
    assert_eq!(rda2_coll[1].contents.len(), 1);

    Ok(())
}


#[test]
fn rdadapter2_showdot() -> io::Result<()> {
    // Note about "?" below: failures to create tempdir/files are beyond the scope of this package, so it's ok
    let dir_1 = tempfile::tempdir()?;
    create_dirstructure(&dir_1.path(), &dirstruct_f2_d1(), None, None)?;

    // Get iterator
    // Need to consume it ONCE otherwise will get error[E0382]: use of moved value: `rda1_all`
    // let rda1_all: RDAdapter1 = RDAdapter1::new(fs_readdir).collect();
    let rda2_iter = RDAdapter2::new(dir_1.path(), true); // <<<< display_all = true
    //let rda2_coll = rda2_iter.collect::<Vec<Vec<PathBufWrap>>>();
    let rda2_coll = rda2_iter.collect::<Vec<Level1Dir>>();

    // 2 files + 1 dir
    assert_eq!(rda2_coll.len(), 2);
    assert_eq!(rda2_coll[0].contents.len(), 3);
    assert_eq!(rda2_coll[1].contents.len(), 2); // this time the dot file is shown

    Ok(())
}
