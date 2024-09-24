mod utils;
mod languages;
mod file_stats;

use clap::Parser;
use std::path::Path;

#[derive(Parser)]
#[command(name = "rloc")]
#[command(author = "Fedi Nabli")]
#[command(version = "1.0.0")]
#[command(about = "Counts lines of code in files and directories")]
struct Cli {
  /// Files or directories to count lines in
  #[arg(short, long, required = true, num_args = 1.., value_name = "FILES", help = "Files or directories to count lines in")]
  files: Vec<String>,
}

fn main() {
  let args = Cli::parse();
  let lang_map = languages::get_languages_map();

  for input in args.files {
    let path = Path::new(&input);

    if path.is_file() {
      utils::process_file(path, &lang_map);
    } else if path.is_dir() {
      utils::process_directory(path, &lang_map);
    } else {
      println!("{} is neither a valid file not directory path", input);
    }
  }
}
