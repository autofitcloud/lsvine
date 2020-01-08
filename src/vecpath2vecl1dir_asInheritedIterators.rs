/// THIS IS A RE-WRITE OF vecpath2vecl1dir.rs such that it's classes and inheritance and iterators
/// DOesn't WORK ATM WITH WEIRD ERRORS

// https://doc.rust-lang.org/std/cmp/fn.min.html
use std::cmp;
use std::{fs, io};
use std::path::PathBuf;

// local imports
pub use crate::level1dir;
use level1dir::Level1Dir;


// Utility functions for Option::and_then
// file_name returns Option: https://doc.rust-lang.org/std/option/index.html
fn to_str(x: &std::ffi::OsStr) -> Option<&str> { x.to_str() }
fn map_collect(x: std::fs::ReadDir) -> Result<Vec<PathBuf>, io::Error> {
   x.map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()
}


struct Converter {
  // input
  level1_paths: Vec<PathBuf>,

  // Collect the data structure in here
  // Each entry corresponds to a folder in the current directory (here-on called "root").
  // The first entry is for files in root, the second is the first child directory, etc.
  level1_dirs: Vec<Level1Dir>,

  // utility variable pointing to the first entry in the Vec, i.e. 0
  idx_root: usize,

  // pointer to index in Vec of current dir
  idx_dir: usize

}


impl ConverterL1 {
  pub fn new(level1_paths: Vec<PathBuf>) -> Converter {
    Converter {
      level1_paths,
      level1_dirs: Vec::new(),
      idx_root: 0,
      idx_dir: 0
    }
  }

  pub fn iterate(&mut self) -> Result<Vec<Level1Dir>, io::Error> {
      if self.level1_paths.is_empty() {
        return Ok(Vec::new());
      }

      // Start by inserting entry for root
      // https://doc.rust-lang.org/book/ch05-01-defining-structs.html
      // Cargo warns to remove the mutability of rootdir. Not sure why.
      let rootdir = Level1Dir {
                         dirname: String::from("."),
                         contents: Vec::new(),
                         max_name_len: 1 // length of "."
                     };
      self.level1_dirs.push(rootdir);

      // println!("l1dirs {:?}", self.level1_dirs[0].contents);

      // loop over all paths in level 1
      // http://blog.ssokolow.com/archives/2017/06/23/rust-looping-on-a-member-variable-without-mutably-borrowing-self/
      for tip_fp in &mut self.level1_paths {
            match tip_fp.file_name().and_then(to_str) {
              Some(tip_fn) => {
                self.got_filename(&tip_fp, tip_fn);
              },
              None => {
                println!("Error reading a path filename. Skipping it.");
                continue;
              }
            }
      }

      Result::Ok(self.level1_dirs)
  }
}

#derived ConverterL1
impl ConverterL2 {

  fn iterate(&mut self) {
              // get yield from parent
              // , tip_fp: &PathBuf, tip_fn: &str
              let tip_fp, tip_fn = super.iterate();

              // debug
              // println!("debug fn {}, is_file {}, is_dir {}", tip_fn, tip_fp.is_file(), tip_fp.is_dir());

              // skip filenames that start with .  
              if tip_fn.starts_with('.') {
                return;
              }

              // display
              // println!("{}", tip_fp.display());

              // filename length
              let tip_nl = tip_fn.chars().count();

              // if path doesn't exist
              if !tip_fp.is_file() && !tip_fp.is_dir() {
                println!("Path doesnt exist: {}. Skipping", tip_fn);
                return;
              }

              // if path is file not dir, put in the root dir
              if tip_fp.is_file() && !tip_fp.is_dir() {
                // append to vector of paths
                self.level1_dirs[self.idx_root].contents.push(tip_fp.to_path_buf());

                // update running maximum path name length
                self.level1_dirs[self.idx_root].max_name_len = cmp::max(self.level1_dirs[self.idx_root].max_name_len, tip_nl);

                return;
              }


              // create a new Level1Dir instance to store data about this level 1 directory
              // Cargo warns to remove the mutability of tip_ld. Not sure why.
              let tip_ld = Level1Dir { dirname: String::from(tip_fn), contents: Vec::new(), max_name_len: tip_nl };

              // insert row for directory: http://phsym.github.io/prettytable-rs/master/prettytable/struct.Table.html
              self.idx_dir = self.level1_dirs.len();
              self.level1_dirs.push(tip_ld);

              // get level 2 paths inside the level 1 directory
              match fs::read_dir(tip_fp).and_then(map_collect) {
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


  fn got_readdir(&mut self, level2_paths: Vec<PathBuf>) {
              // skip the rest of this loop if empty dir
              if level2_paths.is_empty() {
                return;
              }

              // sort because read_dir doesn't guarantee sorted order
              level2_paths.sort();

              // loop over all paths inside the level 1 directory
              for path_fp in level2_paths {
                  // if starts with .  
                  // file_name returns Option: https://doc.rust-lang.org/std/option/index.html
                  match path_fp.file_name().and_then(to_str) {
                      Some(path_fn) => {
                          let path_fl = path_fn.chars().count();
        
                          // skip files starting with .
                          if path_fn.starts_with('.') {
                            continue;
                          }
        
                          // append
                          self.level1_dirs[self.idx_dir].contents.push(path_fp);
        
                          // update max filename length
                          self.level1_dirs[self.idx_dir].max_name_len = cmp::max(
                            self.level1_dirs[self.idx_dir].max_name_len,
                            path_fl
                          );
        
                          // display
                          // println!("{}", path_fp.display())
        
                      },
                      None => {
                        println!("Error reading a path filename. Skipping it.");
                        continue;
                      }
                  }

              }
  }

}



pub fn vecpath2vecl1dir(level1_paths: Vec<PathBuf>) -> Result<Vec<Level1Dir>, io::Error> {
  let mut c = Converter::new(level1_paths);
  c.iterate()
}
