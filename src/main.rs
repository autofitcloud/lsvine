/// Usage: cargo run path/to/dir

use structopt::StructOpt;
use std::process;
///use failure::ResultExt;
///use exitfailure::ExitFailure;
use std::{fs, io};

// https://crates.io/crates/prettytable-rs
// http://phsym.github.io/prettytable-rs/master/prettytable/struct.Table.html
//#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};
use prettytable::format;

// https://eminence.github.io/terminal-size/doc/terminal_size/index.html
// use terminal_size::{Width, Height, terminal_size};
use termion::terminal_size;

// https://doc.rust-lang.org/std/cmp/fn.min.html
use std::cmp;

// coloring dirs
use colored::*;

/// Display contents of directory in vine-like output.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str), default_value = ".")]
    path: std::path::PathBuf,
}


/// Hold data about a level 1 directory and its immediate child paths
struct Level1Dir {
  dirname: String,
  contents: Vec<std::path::PathBuf>,
  max_name_len: usize
}


/// Wrap a prettytable::Table into a flushable Table based on reaching terminal width
/// TODO spinoff a new class TableColFirst about a column-first Table implementation
/// as opposed to the row-first Table implementation in prettytable
/// (which makes sense similar to html table.tr.td order)
// https://doc.rust-lang.org/1.30.0/book/second-edition/ch17-01-what-is-oo.html
struct TableBuf {
  // displayable table in CLI
  table: Table,

  // row titles pertaining to self.table
  row_titles: Vec<String>,

  // number of columns expected in total
  ncol_max: usize,

  // number of columns so far
  ncol_received: usize,

  // terminal width
  terminal_width: usize,

  // number of tables displayed
  idx_table: usize,

  // cumulative sum of width
  maxlen_cum: usize,

  // last directory's maximum pathname length
  maxlen_last: usize,

  // ...
  ncol_flushed: usize
}

impl TableBuf {
  pub fn new(terminal_width: usize, ncol_max: usize) -> TableBuf {
    let mut x = TableBuf {
      table: Table::new(),
      row_titles: Vec::new(),
      ncol_max: ncol_max,
      ncol_received: 0,
      terminal_width: terminal_width,
      idx_table: 0,
      maxlen_cum: 0,
      maxlen_last: 0,
      ncol_flushed: 0
    };

    // disable line separators
    x.table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    return x
  }

  pub fn push(&mut self, l1dir: &Level1Dir) {
      self.ncol_received = self.ncol_received + 1;

      // save some attributes pertaining to the current directory
      self.row_titles.push(l1dir.dirname.blue().bold().to_string()); // .as_str()
      self.maxlen_last = l1dir.max_name_len;

      // add
      if self.ncol_received < self.ncol_max {
        self.maxlen_cum = self.maxlen_cum + l1dir.max_name_len + 3; // add 3 characters
        //println!("max cum {}, term wid {}", maxlen_cum, self.terminal_width);
      }

      let n_l2paths = l1dir.contents.len();
      //println!("add col, {}, n files {}", l1dir.contents.map(|res| res.dirname).collect(), n_l2paths);

      // nothing to do if dir is empty
      if n_l2paths==0 {
        return;
      }

      // iterate over level 2 paths
      for j in 0..n_l2paths {
        if j >= self.table.len() {
          self.table.add_empty_row();
        }

        // Insert empty cells if needed (since this is a column-first table)
        // Note the +1 because the current contents themselves also get inserted via add_cell,
        // so no need for another add_cell("") here
        let ncol_buffer = self.ncol_received - self.ncol_flushed;
        let ncol_inrow = self.table[j].len();
        if ncol_buffer >  ncol_inrow + 1 {
          for _k in ncol_inrow + 1 .. ncol_buffer {
            self.table[j].add_cell(Cell::new(""));
          }
        }

        //println!("level1_vine {} {}", self.table.len(), self.table[j].len());
        let cell_val1 = l1dir.contents[j].file_name().unwrap().to_str().unwrap();
        let cell_val2 = if !l1dir.contents[j].is_file() { cell_val1.blue().bold() } else { cell_val1.normal() };
        self.table[j].add_cell(Cell::new(cell_val2.to_string().as_str()));
      }
  }

  pub fn should_flush(&self) -> bool {
    return self.maxlen_cum >= self.terminal_width;
  }

  pub fn display(&mut self) {
      // assert self.row_titles.len() == self.ncol_received - self.ncol_flushed

      // set title: https://crates.io/crates/prettytable-rs
      self.table.set_titles(Row::new(self.row_titles.iter().map(|res| Cell::new(res)).collect()));

      // print table
      self.table.printstd();
  }
  
  pub fn flush(&mut self) {
      // reset
      if self.ncol_received < self.ncol_max {
        self.maxlen_cum = self.maxlen_last;
      }

      self.row_titles = Vec::new();

      //println!("n_l1dirs {}", n_l1dirs);

      // set to new table
      self.table = Table::new();
      self.table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

      // increment
      self.idx_table = self.idx_table + 1;
      self.ncol_flushed = self.ncol_received;
  }
}

// -----------------------------------

fn main() -> io::Result<()> {
    // get CLI arg values
    let args = Cli::from_args();

    // exit with non-zero if current path doesn't exist
    if !args.path.exists() {
      println!("Path not found: {}", args.path.display());
      process::exit(1);
    }

    // display and exit with 0 if current path is a file not a directory
    if args.path.is_file() {
      println!("{}", args.path.display());
      process::exit(0);
    }

    // list contents of path
    // method 1: http://stackoverflow.com/questions/26076005/ddg#26084812
    // let level1_paths = fs::read_dir(args.path).unwrap();
    // method 2: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    let mut level1_paths = fs::read_dir(args.path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // exit with zero if current path is empty
    if level1_paths.len()==0 {
      process::exit(0);
    }

    // sort because read_dir doesn't guarantee sorted order
    level1_paths.sort();

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

    // loop over all paths in level 1
    for tip_fp in &level1_paths {
        // skip filenames that start with .  
        // file_name returns Option: https://doc.rust-lang.org/std/option/index.html
        let tip_fn = tip_fp.file_name().unwrap().to_str().unwrap();
        if tip_fn.starts_with(".") {
          continue;
        }

        // display
        // println!("{}", tip_fp.display());

        // filename length
        let tip_nl = tip_fn.chars().count();

        // if path is to file not dir, put in the root dir
        if tip_fp.is_file() {
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
        if level2_paths.len()==0 {
          continue;
        }

        // sort because read_dir doesn't guarantee sorted order
        level2_paths.sort();

        // loop over all paths inside the level 1 directory
        for path_fp in level2_paths {
            // if starts with .  
            // file_name returns Option: https://doc.rust-lang.org/std/option/index.html
            let path_fn = path_fp.file_name().unwrap().to_str().unwrap();
            let path_fl = path_fn.chars().count();

            // skip files starting with .
            if path_fn.starts_with(".") {
              continue;
            }

            // append
            level1_dirs[idx_dir].contents.push(path_fp);

            // update max filename length
            level1_dirs[idx_dir].max_name_len = cmp::max(level1_dirs[idx_dir].max_name_len, path_fl);

            // display
            // println!("{}", path_fp.display())
        }

    }

    // get n rows and cols
    let n_l1dirs = level1_dirs.len();

    // This will never match since we always at least have the "." folder
    if n_l1dirs==0 {
      println!("No results");
      process::exit(2);
    }

    // get terminal width .. surely there is a better way
    let _terminal_width = terminal_size().unwrap().0 as usize;
    // println!("terminal width 1 {}", _terminal_width);

    // ---------------------------------

    // convert datastructure into a displayable table with buffering to fill the terminal width
    let mut level1_vine = TableBuf::new(_terminal_width, n_l1dirs);

    // loop over level 1 directories, with 1 extra step in order to flush the vine if it hasn't reached terminal width yet
    for i in 0..n_l1dirs+1 {
      // debug
      //println!("i {}, n_l1dirs {}, idx_table {}, level1_dirs.len {}", i, n_l1dirs, idx_table, level1_dirs.len());

      // if need to flush current level1_vine
      if i==n_l1dirs || level1_vine.should_flush() {
        level1_vine.display();
        level1_vine.flush();
      }

      // this is the extra step required to flush even if the terminal width is not reached
      if i==n_l1dirs { break; }

      level1_vine.push(&level1_dirs[i]);
    }

    // print table
//    level1_vine.table.printstd();

    Ok(())
}
