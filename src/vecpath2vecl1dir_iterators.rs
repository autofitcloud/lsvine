/// THIS IS A RE-WRITE OF vecpath2vecl1dir.rs such that it's classes and inheritance and iterators
/// DOesn't WORK ATM WITH WEIRD ERRORS

// https://doc.rust-lang.org/std/cmp/fn.min.html
use std::cmp;
// use std::{fs, io};
use std::path::PathBuf;

// local imports
pub use crate::level1dir;
use level1dir::Level1Dir;

// -----------------------------------
/// Utility struct to hold PathBuf and its filename as str
/// cannot implement trait Copy due to PathBuf and String
/// #[derive(Copy, Clone)]
pub struct PathBufWrap {
  pub path_buf: PathBuf,
  fn_len: usize, // FIXME why not i32, am I just getting the length of the pointer or of the string itself? Check OsStr versus OsString https://doc.rust-lang.org/std/ffi/struct.OsStr.html
  file_name: String
}

impl PathBufWrap {
  pub fn new(p: PathBuf) -> PathBufWrap {
     let fn_opt = p.file_name().and_then(|x| x.to_str() );
     if fn_opt.is_none() {
       return PathBufWrap {
         path_buf: p,
         file_name: "".to_string(),
         fn_len: 0
       };
     }

     // safe to unwrap since skipped above
     let file_name = fn_opt.unwrap().to_string();
     let mut x = PathBufWrap {
       path_buf: p,
       // file_name: p.file_name().and_then(|&x| x.to_str() ),
       file_name: file_name,
       fn_len: 0 // cannot directly do this // file_name.len()
     };
     x.fn_len = x.file_name.len();
     x
  }
}

// ------------------------------------

/// DEPRECATED in favor of RDAdapter1
/// An iterator adapter that takes an iterator std::fs::ReadDir and:
/// - Consumes it into a collection of DirEntry https://doc.rust-lang.org/std/fs/struct.DirEntry.html
/// - Maps them to PathBuf
/// - skips paths whose filename starts with '.'
/// - skips paths that don't exist on disk
/// - converts it back to an iterator of PathBuf
///   (to stay in the iterator world later and use inheritance on the class's
///    iteration function rather than deal with collections)
///
/// Links
/// https://doc.rust-lang.org/std/fs/struct.ReadDir.html
/// https://doc.rust-lang.org/std/iter/index.html#adapters
pub fn transform_readdir(fs_readdir: std::fs::ReadDir) -> impl Iterator<Item = PathBufWrap> {
    // list contents of path
    // method 1: http://stackoverflow.com/questions/26076005/ddg#26084812
    // let level1_paths = fs::read_dir(args.path).unwrap();
    // method 2: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    // TODO use partition instead of collect
    // https://www.reddit.com/r/rust/comments/eleleu/my_first_cli_in_rust_lsvine_list_contents_of/fditvjp
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.partition
    // Check the docs even/odd example
    let level1_paths = fs_readdir
                             .filter(|res| res.as_ref().ok().is_some()) // quietly skip erroneous entries
                             .map(|e| e.unwrap().path())
                             ;

    // map to PathBufWrap containing filenames
    let level1_pathbufwrap = level1_paths
           // quietly skip None values, like pandas skipna
           .filter(|p| p.file_name().is_some())
           // quietly skip errors
           .filter(|p| p.file_name().and_then(|x| x.to_str() ).is_some())
           // map to filenames (not Option<...>)
           // file_name returns Option: https://doc.rust-lang.org/std/option/index.html
           .map(|p| {
             PathBufWrap::new(p)
           })
           // skip paths filenames that start with .
           .filter(|pbw| return !pbw.file_name.starts_with('.'))
           // skip paths that don't exist on-disk
           .filter(|pbw| {
               if !pbw.path_buf.is_file() && !pbw.path_buf.is_dir() {
                 println!("Path doesnt exist: {}. Skipping", pbw.file_name);
                 return false;
               }
               return true;
           })
           ;

    level1_pathbufwrap
}

// ------------------------------------

/// A re-implementation of transform_readdir but as an iterator adapter
/// for the sake of inheritance later
pub struct RDAdapter1 {
  fs_readdir: std::fs::ReadDir
}

impl RDAdapter1 {
  pub fn new(fs_readdir: std::fs::ReadDir) -> RDAdapter1 {
    RDAdapter1 { fs_readdir }
  }
}

impl Iterator for RDAdapter1 {
  type Item = PathBufWrap;

  fn next(&mut self) -> Option<Self::Item> {
    // list contents of path
    // method 1: http://stackoverflow.com/questions/26076005/ddg#26084812
    // let level1_paths = fs::read_dir(args.path).unwrap();
    // method 2: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    // TODO use partition instead of collect
    // https://www.reddit.com/r/rust/comments/eleleu/my_first_cli_in_rust_lsvine_list_contents_of/fditvjp
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.partition
    // Check the docs even/odd example
    let res1 = self.fs_readdir.next();
    if res1.is_none() { return None; }

    let res2 = res1.unwrap();
    if res2.is_err() { return self.next(); }

    let p = res2.unwrap().path();
    let fn_opt = p.file_name().and_then(|x| x.to_str() );
    if fn_opt.is_none() { return self.next(); }

    // map to PathBufWrap containing filenames
    // re-calculates fn_opt
    let mut pbw = PathBufWrap::new(p);
           
    // since skipped None's above, can unwrap
    pbw.fn_len = pbw.file_name.chars().count();

    // skip paths filenames that start with .
    if pbw.file_name.starts_with('.') { return self.next(); }

    // skip paths that don't exist on-disk
    if !pbw.path_buf.is_file() && !pbw.path_buf.is_dir() {
      println!("Path doesnt exist: {}. Skipping", pbw.file_name);
      return self.next();
    }

    Some(pbw)
  }
}

// ------------------------------------

/// An iterator that transforms the iterator RDAdapter1 (of PathBufWrap)
/// into another iterator of Vec<PathBufWrap>
/// where each entry (i.e. each Vec) is either the root-level files
/// or the level-2 files+dirs
pub struct RDAdapter2 {
  started: bool,
  counter: usize,
  root_pbw: PathBufWrap,
  rda1_dir: Vec<PathBufWrap>
}

impl RDAdapter2 {
  pub fn new(root_path: &std::path::Path) -> RDAdapter2 {

    let root_pbw = PathBufWrap::new(root_path.to_path_buf());

    RDAdapter2 {
      started: false,
      counter: 0,
      root_pbw,
      rda1_dir: Vec::new()
    }
  }
}

impl Iterator for RDAdapter2 {
  type Item = Level1Dir; // Vec<PathBufWrap>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.started {
      if self.counter >= self.rda1_dir.len() {
        return None;
      }
    }

    let l1dir = if !self.started { &self.root_pbw } else { &self.rda1_dir[self.counter] };
    let mut max_name_len = l1dir.fn_len;
    let l1_dirname = if !self.started { String::from(".") } else { String::from(l1dir.file_name.clone()) };

    if self.started {
      self.counter+=1;
    }

    // read_dir again, but this time we care about errors
    let fs_readdir_2 = std::fs::read_dir(&l1dir.path_buf);
    if fs_readdir_2.is_err() {
      println!("Failed to read_dir on {}. Skipping", l1dir.file_name);
      return None;
    }

    // get lower-level iterator
    let rda1_iter = RDAdapter1::new(fs_readdir_2.unwrap());

    // if started and already performing directories
    if self.started {
      let mut rda2_all_vec = rda1_iter.collect::<Vec<PathBufWrap>>();

      // sort
      rda2_all_vec.sort_by_key(|pbw| pbw.file_name.clone());

      // calculate max_name_len
      for l in rda2_all_vec.iter().map(|pbw| pbw.fn_len) {
        max_name_len = cmp::max(max_name_len, l);
      }

      // build Level1Dir object
      let mut rda1_contents: Vec<PathBuf> = Vec::new();
      for pbw in rda2_all_vec {
        rda1_contents.push(pbw.path_buf);
      }
      let rda2_l1dir = Level1Dir {
        dirname: l1_dirname,
        contents: rda1_contents,
        max_name_len
      };

      // for all level-1 directories, return both files and directories
      return Some(rda2_l1dir); // rda1_both);
    }

    // if didn't start yet
    let (mut rda1_file, mut rda1_dir) : (Vec<PathBufWrap>, Vec<PathBufWrap>) = rda1_iter.partition(|pbw| pbw.path_buf.is_file());

    // sort because read_dir doesn't guarantee sorted order
    // FIXME
    rda1_file.sort_by_key(|pbw| pbw.file_name.clone());
    rda1_dir.sort_by_key(|pbw| pbw.file_name.clone());

    // save directories for later, and make sure only to save the level-1 directories
    self.rda1_dir = rda1_dir;

    // raise the flag to start on child directories in the next iteration
    self.started = true;

    // calculate max_name_len
    for l in rda1_file.iter().map(|pbw| pbw.fn_len) {
      max_name_len = cmp::max(max_name_len, l);
    }

    // build Level1Dir object
    let mut rda1_contents: Vec<PathBuf> = Vec::new();
    for pbw in rda1_file {
      rda1_contents.push(pbw.path_buf);
    }
    let rda2_l1dir = Level1Dir {
      dirname: l1_dirname,
      contents: rda1_contents,
      max_name_len
    };

    // only return files, no directories
    return Some(rda2_l1dir); // rda1_file
  }
}

// ------------------------------------

/*
/// An iterator adapter transforming an iterator of PathBufWrap to an iterator of Level1Dir
pub struct RDAdapter3 {
  // input iterator
  rda1: RDAdapter1,

  // Collect the data structure in here
  // Each entry corresponds to a folder in the current directory (here-on called "root").
  // The first entry is for files in root, the second is the first child directory, etc.
  level1_dirs: Vec<Level1Dir>,

  // utility variable pointing to the first entry in the Vec, i.e. 0
  idx_root: usize,

  // pointer to index in Vec of current dir
  idx_dir: usize

}

impl RDAdapter3 {
  pub fn new(rda1: RDAdapter1) -> RDAdapter3 {
    let mut x = RDAdapter3 {
      rda1,
      level1_dirs: Vec::new(),
      idx_root: 0,
      idx_dir: 0
    };

    // Start by inserting entry for root
    // https://doc.rust-lang.org/book/ch05-01-defining-structs.html
    // Cargo warns to remove the mutability of rootdir. Not sure why.
    let rootdir = Level1Dir {
                       dirname: String::from("."),
                       contents: Vec::new(),
                       max_name_len: 1 // length of "."
                   };
    x.level1_dirs.push(rootdir);

    x
  }
}

// Cannot derive from RDAdapter1 since Self::Item is different
impl Iterator for RDAdapter3 {
  type Item = Level1Dir;

  fn next(&mut self) -> Option<Self::Item> {
      let pbw = self.rda1.next();

      // if path is file not dir, put in the root dir
      if pbw.path_buf.is_file() && !pbw.path_buf.is_dir() {
        // append to vector of paths
        self.level1_dirs[self.idx_root].contents.push(pbw.path_buf.to_path_buf());

        // update running maximum path name length
        self.level1_dirs[self.idx_root].max_name_len = cmp::max(
          self.level1_dirs[self.idx_root].max_name_len,
          tip_nl
        );

        return;
      }


      // create a new Level1Dir instance to store data about this level 1 directory
      // Cargo warns to remove the mutability of tip_ld. Not sure why.
      let tip_ld = Level1Dir { dirname: String::from(tip_fn), contents: Vec::new(), max_name_len: tip_nl };

      // insert row for directory: http://phsym.github.io/prettytable-rs/master/prettytable/struct.Table.html
      self.idx_dir = self.level1_dirs.len();
      self.level1_dirs.push(tip_ld);

      // get level 2 paths inside the level 1 directory
      match fs::read_dir(pbw.path_buf).and_then(map_collect) {
        Ok(level2_paths) => {
          self.got_readdir(level2_paths)
        },
        Err(e) => {
          println!("Failed to read dir contents. Skipping dir. Error: {}", e);
          return;
        }
      }

  }
}
*/
