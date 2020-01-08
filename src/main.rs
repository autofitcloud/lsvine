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


struct Level1Dir {
  dirname: String,
  contents: Vec<std::path::PathBuf>,
  max_name_len: usize
}


fn main() -> io::Result<()> {
    //println!("Hello, world!");
    let args = Cli::from_args();

    // check
    if !args.path.exists() {
      println!("Path not found: {}", args.path.display());
      process::exit(1);
    }

    // if file
    if args.path.is_file() {
      println!("{}", args.path.display());
      process::exit(1);
    }

    // list contents of path
    // method 1: http://stackoverflow.com/questions/26076005/ddg#26084812
    // let l1 = fs::read_dir(args.path).unwrap();
    // method 2: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    let mut l1 = fs::read_dir(args.path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // if empty
    if l1.len()==0 {
      process::exit(4);
    }

    // sort
    l1.sort();

    // Collect the data structure
    // Each entry corresponds to a folder in the current directory (here-on called "root").
    // The first entry is for files in root, the second is the first child directory, etc.
    let mut table1: Vec<Level1Dir> = Vec::new();

    // Start by inserting entry for root
    // https://doc.rust-lang.org/book/ch05-01-defining-structs.html
    // Cargo warns to remove the mutability of rootdir. Not sure why.
    let idx_root = 0; // table1.len();
    let rootdir = Level1Dir {
                       dirname: String::from("."),
                       contents: Vec::new(),
                       max_name_len: 1 // length of "."
                   };
    table1.push(rootdir);

    // loop
    for tip_fp in &l1 {
        // if starts with .  
        // file_name returns Option: https://doc.rust-lang.org/std/option/index.html
        let tip_fn = tip_fp.file_name().unwrap().to_str().unwrap();
        if tip_fn.starts_with(".") {
          continue;
        }

        // display
        // println!("{}", tip_fp.display());

        // filename length
        let tip_nl = tip_fn.chars().count();

        // if file
        if tip_fp.is_file() {
          // append
          table1[idx_root].contents.push(tip_fp.to_path_buf());

          // update max_name_len
          table1[idx_root].max_name_len = cmp::max(table1[idx_root].max_name_len, tip_nl);

          // done
          continue;
        }

        // new Level1Dir
        // Cargo warns to remove the mutability of tip_ld. Not sure why.
        let tip_ld = Level1Dir { dirname: String::from(tip_fn), contents: Vec::new(), max_name_len: tip_nl };

        // insert row for directory: http://phsym.github.io/prettytable-rs/master/prettytable/struct.Table.html
        let idx_dir = table1.len();
        table1.push(tip_ld);

        // get level 2
        let mut l2 = fs::read_dir(tip_fp)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        // sort
        l2.sort();

        // loop
        for path_fp in l2 {
            // if starts with .  
            // file_name returns Option: https://doc.rust-lang.org/std/option/index.html
            let path_fn = path_fp.file_name().unwrap().to_str().unwrap();
            let path_fl = path_fn.chars().count();

            // skip files starting with .
            if path_fn.starts_with(".") {
              continue;
            }

            // append
            table1[idx_dir].contents.push(path_fp);

            // update max filename length
            table1[idx_dir].max_name_len = cmp::max(table1[idx_dir].max_name_len, path_fl);

            // display
            // println!("{}", path_fp.display())
        }

    }

    // get n rows and cols
    let nrow = table1.len();

    if nrow==0 {
      println!("No results");
      process::exit(2);
    }

    // get terminal width .. surely there is a better way
    let _terminal_width = terminal_size().unwrap().0 as usize;
    // println!("terminal width 1 {}", _terminal_width);

    // save into a displayable table
    let mut table2 = Table::new();
    table2.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    // number of tables displayed
    let mut idx_table = 0;
    // maximum number of columns, will override later when first table is printed
    let mut max_col = 5;
    // cumulative sum of width
    let mut max_cum = 0;
    let mut sum_displayed = 0;
    for i in 0..nrow+1 {
      // debug
      //println!("i {}, nrow {}, idx_table {}, table1.len {}", i, nrow, idx_table, table1.len());
   
      // add
      if i<nrow {
        max_cum = max_cum + table1[i].max_name_len + 3; // add 3 characters
        //println!("max cum {}, term wid {}", max_cum, _terminal_width);
      }
 
      // if need to flush current table2
      // if i==nrow || (i > 0 && i % max_col == 0) {
      if i==nrow || max_cum >= _terminal_width {
        // reset
        if i<nrow {
          max_cum = table1[i].max_name_len;
        }

        // override max_col
        max_col = i - sum_displayed; // i doesnt work because after the first wrap, need to subtract the sum of columns displayed so far // table2.len() is the number of rows .. we need number of columns here
        //println!("max col {}, nrow {}", max_col, nrow);

        // set title: https://crates.io/crates/prettytable-rs
        let row_titles = table1[sum_displayed .. sum_displayed + max_col].iter().map(|res| Cell::new(res.dirname.blue().bold().to_string().as_str())).collect();
        table2.set_titles(Row::new(row_titles));

        // print table
        table2.printstd();

        // set to new table
        table2 = Table::new();
        table2.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        // increment
        idx_table = idx_table + 1;
        sum_displayed = sum_displayed + max_col;
      }

      if i==nrow {
        break
      }

      let ncol = table1[i].contents.len();
      //println!("add col, {}, n files {}", table1[i].map(|res| res.dirname).collect(), ncol);

      if ncol==0 {
        continue
      }
      for j in 0..ncol {
        if j >= table2.len() {
          table2.add_empty_row();
        }

        let idx_start = idx_table*max_col + table2[j].len();
        if i >=  idx_start {
          for _k in idx_start .. i {
            table2[j].add_cell(Cell::new(""));
          }
        }

        //println!("Table2 {} {}", table2.len(), table2[j].len());
        let cell_val1 = table1[i].contents[j].file_name().unwrap().to_str().unwrap();
        let cell_val2 = if !table1[i].contents[j].is_file() { cell_val1.blue().bold() } else { cell_val1.normal() };
        table2[j].add_cell(Cell::new(cell_val2.to_string().as_str()));
      }
    }

    // print table
//    table2.printstd();

    Ok(())
}
