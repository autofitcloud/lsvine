use std::str::Chars;

/// https://beta.rustgym.com/longest-common-prefix/
fn longest_common_prefix(strs: &[String]) -> String {
        let mut prefix = String::new();
        let mut iters: Vec<Chars> = strs.iter().map(|s| {s.chars()}).collect();
        let mut curr_char: Option<char> = None;
        // clippy // if strs.len() < 1 { return prefix }
        if strs.is_empty() { return prefix }
        loop {
            // clippy // curr_char.take().map(|ch| prefix.push(ch));
            if let Some(ch) = curr_char.take() { prefix.push(ch) };
            for iter in iters.iter_mut() {
                let mut ch = iter.next();
                if ch.is_none() { return prefix }
                match curr_char {
                    None => curr_char = ch.take(),
                    Some(curr) => {
                        if curr != ch.unwrap() {
                            return prefix
                        }

                    },
                }
            }
        }

}

#[test]
fn test_longestcommonprefix_ex1() {
  let input_1 = vec!["hello", "hi", "hey"];
  let input_2: Vec<String> = input_1.iter().map(|s| s.to_string()).collect();
  let actual = longest_common_prefix(&input_2);
  let expected = "h";
  assert_eq!(expected, actual);
}

#[test]
fn test_longestcommonprefix_ex2() {
  let input_1 = vec!["hello", "hi", "hey", "another"];
  let input_2: Vec<String> = input_1.iter().map(|s| s.to_string()).collect();
  let actual = longest_common_prefix(&input_2);
  let expected = "";
  assert_eq!(expected, actual);
}


// ------------------------------------------

/// Utility class for Implier
pub struct StringGroup<T: std::string::ToString> {
  // a vector of items that implement the ToString trait
  // https://doc.rust-lang.org/std/string/trait.ToString.html
  // eg std::string::String
  pub l2_obj   : Vec<T>,
  // a vector of the results from to_string for each ToString-able object
  pub l2_string: Vec<std::string::String>,
  // the longest common prefix for the l2_string vector
  pub prefix: String
}

/// A transformer that gathers a vector of strings into groups by common prefixes
/// eg if there are 3 files "f1.md, f1.html" they would show up as "f1.*"
/// Note this requires the vector to already be sorted.
/// Cannot use #[derive(Default)] because PathBufWrap doesn't implement defaults
pub struct Implier<T: std::string::ToString> {
  pub level_1: Vec<StringGroup<T>>,
}

impl<T> Implier<T> where T: std::string::ToString {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Implier<T> {
      Implier {
        level_1: Vec::new()
      }
    }

    fn push_new(&mut self) -> &mut StringGroup<T> {
      self.level_1.push(StringGroup::<T> {
        l2_string: Vec::new(),
        l2_obj: Vec::new(),
        prefix: "".to_string()
      });

      // cannot be mutable // self.level_1.last().unwrap() // safe to unwrap since just pushed above
      let n = self.level_1.len();
      &mut self.level_1[n-1]
    }

    pub fn contract(&mut self, strs_in: Vec<T>, minimum_prefix_length: usize) { // Vec<StringGroup<T>> {
      //if strs_in.len() <= 2 {
      //  panic!("Why contract when vector.len <= 2");
      //}

      // create first group to start appending to it
      let mut group_last: &mut StringGroup<T> = self.push_new();

      // iterate over strings
      for strs_current in strs_in {

        // debugging
        // println!("strs_current {}, group_last.l2_*.len {}", strs_current.to_string(), group_last.l2_string.len());

        // nothing to do with the 1st string
        if group_last.l2_string.is_empty() {
          group_last.l2_string.push(strs_current.to_string());
          group_last.prefix = strs_current.to_string();
          group_last.l2_obj.push(strs_current);
          continue;
        }

        // with 2 strings or more, need to evaluate if ok to gather to last group
        // or to open a new group
        let group_theoretical = [group_last.l2_string.clone(), vec![strs_current.to_string()]].concat();
        let lcp_theoretical = longest_common_prefix(&group_theoretical);

        // debugging
        // println!("lcp_theoretical {}, gruop_last.prefix {}", lcp_theoretical, group_last.prefix);

        // if LCP disappears,
        // or if it is shorter than the minimum prefix length
        // create a new group and start there
        if lcp_theoretical.chars().count() == 0 ||
           lcp_theoretical.chars().count() < minimum_prefix_length
        {
          group_last = self.push_new();
          group_last.prefix = strs_current.to_string();
          group_last.l2_string.push(strs_current.to_string());
          group_last.l2_obj.push(strs_current);
          continue;
        }

        // if LCP is the same, just push to current group and continue
        if lcp_theoretical == group_last.prefix {
          group_last.l2_string.push(strs_current.to_string());
          group_last.l2_obj.push(strs_current);
          // group_last.prefix = group_last.prefix;
          continue;
        }

        // if 2nd entry, overwrite the prefix for sure
        if group_last.l2_string.len() == 1 {
          group_last.l2_string.push(strs_current.to_string());
          group_last.l2_obj.push(strs_current);
          group_last.prefix = lcp_theoretical;
          continue;
        }

        // if LCP is shorter, create a new group and start there
        // Note: keeping this as a separate condition as I can foresee expanding this
        // to allow a prefix string contraction to maintain the same group under some conditions
        if lcp_theoretical.chars().count() < group_last.prefix.chars().count() {
          group_last = self.push_new();
          group_last.l2_string.push(strs_current.to_string());
          group_last.prefix = strs_current.to_string();
          continue;
        }

        // if LCP is longer: this is not possible as long as the longest_common_prefix is working as expected
        // so not writing any code for this
        panic!("Something is wrong with the longest_common_prefix function");
      }

      //let strs_out = self.level_1.iter().map(|sg| {
      //    [sg.prefix.clone(), "*".to_string()].concat()
      //  }).collect();
      //strs_out

      // self.level_1
    }
}

#[test]
fn test_implier_contract() {
  let input_1 = vec!["hello", "hey", "hi"];
  let input_2: Vec<String> = input_1.iter().map(|s| s.to_string()).collect();
  let mut i = Implier::new();
  i.contract(input_2, 1);
  let actual_1 = i.level_1;
  let actual_2: Vec<String> = actual_1.iter().map(|sg| sg.prefix.clone()).collect();
  let expected = vec!["he", "hi"];
  assert_eq!(expected, actual_2);
}

// ------------------------------------------

// use crate::vecpath2vecl1dir_iterators::PathBufWrap;

