/// Usage: cargo run path/to/dir

use structopt::StructOpt;
use std::process;
///use failure::ResultExt;
///use exitfailure::ExitFailure;
use std::{fs, io};

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

    // list contents of path
    // method 1: http://stackoverflow.com/questions/26076005/ddg#26084812
    // let l1 = fs::read_dir(args.path).unwrap();
    // method 2: https://doc.rust-lang.org/std/fs/fn.read_dir.html
    let mut l1 = fs::read_dir(args.path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // sort
    l1.sort();

    // loop
    for tip in l1 {
        // if starts with .  
        // file_name returns Option: https://doc.rust-lang.org/std/option/index.html
        if tip.file_name().unwrap().to_str().unwrap().starts_with(".") {
          continue;
        }

        // display
        println!("{}", tip.display());

        // if file
        if tip.is_file() {
          continue;
        }

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

            // display
            println!("{}", path.display())
        }

    }


    Ok(())
}
