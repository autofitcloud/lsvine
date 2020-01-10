/// Usage: cargo run path/to/dir

use structopt::StructOpt;
use std::process;
///use failure::ResultExt;
///use exitfailure::ExitFailure;
// use std::{fs, io};
use std::io;


// https://docs.rs/termion/1.5.4/termion/fn.terminal_size.html
use termion::terminal_size;

// imports
pub mod tablebuf;
use tablebuf::TableBuf;
pub mod level1dir;
use level1dir::Level1Dir;

pub mod vecpath2vecl1dir_iterators;
//use vecpath2vecl1dir_iterators::{PathBufWrap, RDAdapter2};
use vecpath2vecl1dir_iterators::RDAdapter2;

// ---------------------------------

/// Display contents of directory in vine-like output.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str), default_value = ".")]
    path_buf: std::path::PathBuf,
}


// -----------------------------------

fn main() -> io::Result<()> {
    // get CLI arg values
    let args = Cli::from_args();

    // exit with non-zero if current path doesn't exist
    if !args.path_buf.exists() {
      println!("Path not found: {}", args.path_buf.display());
      process::exit(1);
    }

    // display and exit with 0 if current path is a file not a directory
    if args.path_buf.is_file() {
      println!("{}", args.path_buf.display());
      process::exit(0);
    }

    // Collect the data structure
    // Each entry corresponds to a folder in the current directory (here-on called "root").
    // The first entry is for files in root, the second is the first child directory, etc.
    let rda2_iter = RDAdapter2::new(args.path_buf.as_path());
    //let level1_dirs = rda2_iter.collect::<Vec<Vec<PathBufWrap>>>();
    let level1_dirs = rda2_iter.collect::<Vec<Level1Dir>>();

    // exit with zero if current path is empty
    if level1_dirs.is_empty() {
      process::exit(0);
    }

    // get n rows and cols
    let n_l1dirs = level1_dirs.len();

    // This will never match since we always at least have the "." folder
    if n_l1dirs==0 {
      println!("No results");
      process::exit(2);
    }

    // get terminal width .. surely there is a better way
    let mut _terminal_width = 100; // default
    match terminal_size() {
      Ok(size) => {
        _terminal_width = size.0 as usize;
      },
      Err(_e) => {
        // do nothing as didn't find terminal width
        println!("Warning: couldn't determine terminal width. Assuming {}", _terminal_width);
      }
    }

    // ---------------------------------

    // convert datastructure into a displayable table with buffering to fill the terminal width
    let mut level1_vine = TableBuf::new(_terminal_width, n_l1dirs);

    // loop over level 1 directories, with 1 extra step in order to flush the vine if it hasn't reached terminal width yet
    for l1dir in level1_dirs.iter() {
      // debug
      //println!("i {}, n_l1dirs {}, idx_table {}, level1_dirs.len {}", i, n_l1dirs, idx_table, level1_dirs.len());

      // if need to flush current level1_vine
      if level1_vine.should_flush(&l1dir) {
        level1_vine.display();
        level1_vine.flush();
      }

      // push to table
      level1_vine.push(&l1dir);
    }

    // one final display before the end of the program
    if !level1_vine.table.is_empty() {
        level1_vine.display();
        level1_vine.flush();
    }

    // print table
//    level1_vine.table.printstd();

    Ok(())
}
