// https://doc.rust-lang.org/std/cmp/fn.min.html
use std::cmp;
use std::{fs, io};

// local imports
pub use crate::level1dir;
use level1dir::Level1Dir;


// file_name returns Option: https://doc.rust-lang.org/std/option/index.html
pub fn to_str(x: &std::ffi::OsStr) -> Option<&str> { x.to_str() }


pub fn vecpath2vecl1dir(level1_paths: Vec<std::path::PathBuf>) -> Result<Vec<Level1Dir>, io::Error> {
    if level1_paths.is_empty() {
      return Ok(Vec::new());
    }

    // Collect the data structure
    // Each entry corresponds to a folder in the current directory (here-on called "root").
    // The first entry is for files in root, the second is the first child directory, etc.
    let mut level1_dirs: Vec<Level1Dir> = Vec::new();

    // Start by inserting entry for root
    // https://doc.rust-lang.org/book/ch05-01-defining-structs.html
    // Cargo warns to remove the mutability of rootdir. Not sure why.
    let idx_root = 0; // level1_dirs.len();
    let rootdir = Level1Dir {
                       dirname: String::from("."),
                       contents: Vec::new(),
                       max_name_len: 1 // length of "."
                   };
    level1_dirs.push(rootdir);

    // println!("l1dirs {:?}", level1_dirs[0].contents);

    // loop over all paths in level 1
    for tip_fp in &level1_paths {
        match tip_fp.file_name().and_then(to_str) {
          Some(tip_fn) => {

              // debug
              // println!("debug fn {}, is_file {}, is_dir {}", tip_fn, tip_fp.is_file(), tip_fp.is_dir());

              // skip filenames that start with .  
              if tip_fn.starts_with('.') {
                continue;
              }

              // display
              // println!("{}", tip_fp.display());

              // filename length
              let tip_nl = tip_fn.chars().count();

              // if path doesn't exist
              if !tip_fp.is_file() && !tip_fp.is_dir() {
                println!("Path doesnt exist: {}. Skipping", tip_fn);
                continue;
              }

              // if path is file not dir, put in the root dir
              if tip_fp.is_file() && !tip_fp.is_dir() {
                // append to vector of paths
                level1_dirs[idx_root].contents.push(tip_fp.to_path_buf());

                // update running maximum path name length
                level1_dirs[idx_root].max_name_len = cmp::max(level1_dirs[idx_root].max_name_len, tip_nl);

                continue;
              }


              // create a new Level1Dir instance to store data about this level 1 directory
              // Cargo warns to remove the mutability of tip_ld. Not sure why.
              let tip_ld = Level1Dir { dirname: String::from(tip_fn), contents: Vec::new(), max_name_len: tip_nl };

              // insert row for directory: http://phsym.github.io/prettytable-rs/master/prettytable/struct.Table.html
              let idx_dir = level1_dirs.len();
              level1_dirs.push(tip_ld);

              // get level 2 paths inside the level 1 directory
              let mut level2_paths = fs::read_dir(tip_fp)?
                  .map(|res| res.map(|e| e.path()))
                  .collect::<Result<Vec<_>, io::Error>>()?;

              // skip the rest of this loop if empty dir
              if level2_paths.is_empty() {
                continue;
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
                          level1_dirs[idx_dir].contents.push(path_fp);
        
                          // update max filename length
                          level1_dirs[idx_dir].max_name_len = cmp::max(
                            level1_dirs[idx_dir].max_name_len,
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

          },
          None => {
            println!("Error reading a path filename. Skipping it.");
            continue;
          }
        }


    }

    Result::Ok(level1_dirs)
}

