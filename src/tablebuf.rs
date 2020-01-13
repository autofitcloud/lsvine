// https://crates.io/crates/prettytable-rs
// http://phsym.github.io/prettytable-rs/master/prettytable/struct.Table.html
//#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};
use prettytable::format;

// coloring dirs
use colored::*;


// local imports
pub use crate::level1dir;

/// Wrap a prettytable::Table into a flushable Table based on reaching terminal width
/// TODO spinoff a new class TableColFirst about a column-first Table implementation
/// as opposed to the row-first Table implementation in prettytable
/// (which makes sense similar to html table.tr.td order)
// https://doc.rust-lang.org/1.30.0/book/second-edition/ch17-01-what-is-oo.html
pub struct TableBuf {
  // displayable table in CLI
  pub table: Table,

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
      ncol_max, // equivalent to ncol_max: ncol_max
      ncol_received: 0,
      terminal_width,
      idx_table: 0,
      maxlen_cum: 0,
      maxlen_last: 0,
      ncol_flushed: 0
    };

    // disable line separators
    x.table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    x
  }

  pub fn push(&mut self, l1dir: &level1dir::Level1Dir) {
      self.ncol_received += 1;

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
        match l1dir.contents[j].file_name().and_then(|x| x.to_str()) {
            Some(cell_val1) => {
                let cell_val2 = if !l1dir.contents[j].is_file() { cell_val1.blue().bold() } else { cell_val1.normal() };
                self.table[j].add_cell(Cell::new(cell_val2.to_string().as_str()));
            },
            None => {
              println!("Error reading a path filename. Skipping it.");
              self.table[j].add_cell(Cell::new(""));
              continue;
            }
        }


      }
  }

  pub fn should_flush(&self, l1dir: &level1dir::Level1Dir) -> bool {
    self.maxlen_cum + l1dir.max_name_len >= self.terminal_width
  }

  pub fn display(&mut self) {
      // assert self.row_titles.len() == self.ncol_received - self.ncol_flushed

      // set title: https://crates.io/crates/prettytable-rs
      self.table.set_titles(Row::new(self.row_titles.iter().map(|res| Cell::new(res)).collect()));

      // print table
      // Update 2020-01-13
      //     prettytable::Table::printstd panics when piped into "head"
      //     Replace with prettytable::Table::print
      //     https://github.com/phsym/prettytable-rs/issues/103
      // self.table.printstd();
      let mut out = std::io::stdout();
      let _ = self.table.print(&mut out);  // ignoring the Result<T, E> here
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
      self.idx_table += 1;
      self.ncol_flushed = self.ncol_received;
  }
}
