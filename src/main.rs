/// Usage: cargo run path/to/dir

use structopt::StructOpt;
use std::process;
///use failure::ResultExt;
///use exitfailure::ExitFailure;
use std::{fs, io};

// https://crates.io/crates/prettytable-rs
//#[macro_use] extern crate prettytable;
use prettytable::{Table, Cell}; // , Row
use prettytable::format;

// https://eminence.github.io/terminal-size/doc/terminal_size/index.html
use terminal_size::{Width, Height, terminal_size};

/// Display contents of directory in vine-like output.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str), default_value = ".")]
    path: std::path::PathBuf,
}


//fn doit(path: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
//    if !path.exists() {
//      println!("Path not found: {}", args.path.display());
//      process::exit(1);
//    //  panic!("Path not found: {}", args.path.display());
//    }

    //if !path.exists() {
    //   return Err("Not found");
    //}
    //Ok(())
//}


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
    let mut table1 = Table::new();

    // list contents of path
    // method 1: http://stackoverflow.com/questions/26076005/ddg#26084812
    // let l1 = fs::read_dir(args.path).unwrap();
    // method 2: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    let mut l1 = fs::read_dir(args.path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // sort
    l1.sort();

    // insert row for root: http://phsym.github.io/prettytable-rs/master/prettytable/struct.Table.html
    let idx_root = table1.len(); // 0;
    table1.add_empty_row();

    // loop
    for tip in l1 {
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
          table1[idx_root].add_cell(Cell::new(tip.file_name().unwrap().to_str().unwrap()));

          // done
          continue;
        }

        // insert row for directory: http://phsym.github.io/prettytable-rs/master/prettytable/struct.Table.html
        let idx_dir = table1.len();
        table1.add_empty_row();

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
            table1[idx_dir].add_cell(Cell::new(path.file_name().unwrap().to_str().unwrap()));

            // display
            // println!("{}", path.display())
        }

    }

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

    // get terminal width
    let size = terminal_size();
    let mut _terminal_width = 0;
    if let Some((Width(w), Height(_h))) = size {
        _terminal_width = w;
    }

    // transpose the table
    let mut table2 = Table::new();
    table2.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    // number of tables displayed
    let mut idx_table = 0;
    for i in 0..nrow {
    
      // if need to flush current table2
      let max_col = 5;
      if i > 0 && i % max_col == 0 {
        // print table
        table2.printstd();

        // set to new table
        table2 = Table::new();
        table2.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        // increment
        idx_table = idx_table + 1;
      }


      let ncol = table1[i].len();
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

        // println!("Table2 {} {}", table2.len(), table2[j].len());
        table2[j].add_cell(Cell::new(table1[i][j].get_content().as_str()));
      }
    }

    // print table
    table2.printstd();

    Ok(())
}
