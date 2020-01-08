/// Hold data about a level 1 directory and its immediate child paths
pub struct Level1Dir {
  pub dirname: String,
  pub contents: Vec<std::path::PathBuf>,
  pub max_name_len: usize
}
