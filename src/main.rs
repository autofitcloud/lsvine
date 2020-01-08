/// Usage: cargo run path/to/dir

use structopt::StructOpt;
use std::process;
///use failure::ResultExt;
///use exitfailure::ExitFailure;
use std::{fs, io};

// https://crates.io/crates/prettytable-rs
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

    // Create the table. Each row corresponds to a folder. The first row is root, the second is the first directory.
    // Note that the end-goal requires transposing this table, but it's just easier to deal with this table as such and later transpose.
    // let mut table1 = Table::new();
    let mut table1: Vec<Vec<std::path::PathBuf>> = Vec::new();

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

    // insert row for root: http://phsym.github.io/prettytable-rs/master/prettytable/struct.Table.html
    let idx_root = table1.len(); // 0;
    //table1.add_empty_row();
    table1.push(Vec::new());

    // gather dir names
    let mut dirnames = Vec::new();
    dirnames.push(".");

    // loop
    for tip in &l1 {
        // if starts with .  
        // file_name returns Option: https://doc.rust-lang.org/std/option/index.html
        if tip.file_name().unwrap().to_str().unwrap().starts_with(".") {
          continue;
        }

        // display
        // println!("{}", tip.display());

        // if file
        if tip.is_file() {
          // append
          //table1[idx_root].add_cell(Cell::new(tip));
          table1[idx_root].push(tip.to_path_buf());

          // done
          continue;
        }

        // push dirname
        dirnames.push(tip.file_name().unwrap().to_str().unwrap());

        // insert row for directory: http://phsym.github.io/prettytable-rs/master/prettytable/struct.Table.html
        let idx_dir = table1.len();
        //table1.add_empty_row();
        table1.push(Vec::new());

        // get level 2
        let mut l2 = fs::read_dir(tip)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        // sort
        l2.sort();

        // loop
        for path in l2 {
            // if starts with .  
            // file_name returns Option: https://doc.rust-lang.org/std/option/index.html
            if path.file_name().unwrap().to_str().unwrap().starts_with(".") {
              continue;
            }

            // append
            //table1[idx_dir].add_cell(Cell::new(path));
            table1[idx_dir].push(path);

            // display
            // println!("{}", path.display())
        }

    }

    // get maximum file length per row in table1, in cumulative sum .. no longer in cum sum
    // TODO Isn't this stuff that used to come from pandas/numpy in python?
    // Need to factor out into a library or find something like
    // (example) https://docs.rs/ndarray/0.13.0/ndarray/index.html
    // (old)     https://www.reddit.com/r/rust/comments/5ks00y/introducing_utah_a_rust_dataframe/
    // (new)     https://www.reddit.com/r/rust/comments/apo66e/dataframes_what_do_you_need/
    let mut max_all: Vec<usize> = Vec::new();
    for i in 0..table1.len() {
      let mut max_cur = dirnames[i].chars().count(); // initialize to column header
      //println!("max filename length in {} = {} ...", dirnames[i], max_cur);

      for j in 0..table1[i].len() {
        let max_cell = table1[i][j].file_name().unwrap().to_str().unwrap().chars().count();
        max_cur = cmp::max(max_cur, max_cell);
        //println!("new filename {} \t\t max cur {} \t max cell {}", table1[i][j].get_content(), max_cur, max_cell);
      }
      // let max_pre = if max_all.len()==0 { 0 } else { max_all[max_all.len()-1] };
      //println!("max pre {}, max cur {}", max_pre, max_cur);
      // max_all.push(max_pre + max_cur + 3); // always add 3 characters
      max_all.push(max_cur);
    }
    //println!("{:?}", max_all);

    // print table
    // table1.printstd();

    // get n rows and cols
    let nrow = table1.len();
//    let mut ncol = 0;
//    for i in 0..nrow {
//      if table1[i].len() > ncol {
//        ncol = table1[i].len()
//      }
//    }

    if nrow==0 {
      println!("No results");
      process::exit(2);
    }

    // get terminal width .. surely there is a better way
    let _terminal_width = terminal_size().unwrap().0 as usize;
    // println!("terminal width 1 {}", _terminal_width);

    // transpose the table
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
      //println!("i {}, nrow {}, idx_table {}, dirnames.len {}", i, nrow, idx_table, dirnames.len());
   
      // add
      if i<nrow {
        max_cum = max_cum + max_all[i] + 3; // add 3 characters
        //println!("max cum {}, term wid {}", max_cum, _terminal_width);
      }
 
      // if need to flush current table2
      // if i==nrow || (i > 0 && i % max_col == 0) {
      if i==nrow || max_cum >= _terminal_width {
        // reset
        if i<nrow {
          max_cum = max_all[i];
        }

        // override max_col
        max_col = i - sum_displayed; // i doesnt work because after the first wrap, need to subtract the sum of columns displayed so far // table2.len() is the number of rows .. we need number of columns here
        //println!("max col {}, nrow {}", max_col, nrow);

        // set title: https://crates.io/crates/prettytable-rs
        // slicing // let row_titles = l1[idx_table*max_col .. (idx_table+1)*max_col-1].to_vec().iter().map(|res| Cell::new(res.file_name().unwrap().to_str().unwrap())).collect();
        let row_titles = dirnames[sum_displayed .. sum_displayed + max_col].to_vec().iter().map(|res| Cell::new(res.blue().bold().to_string().as_str())).collect();
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

      let ncol = table1[i].len();
      //println!("add col, {}, n files {}", dirnames[i], ncol);

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
        let cell_val1 = table1[i][j].file_name().unwrap().to_str().unwrap();
        let cell_val2 = if !table1[i][j].is_file() { cell_val1.blue().bold() } else { cell_val1.normal() };
        table2[j].add_cell(Cell::new(cell_val2.to_string().as_str()));
      }
    }

    // print table
//    table2.printstd();

    Ok(())
}
