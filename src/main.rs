mod utils;
mod languages;
mod file_stats;

use std::fs;
use clap::Parser;
use std::path::Path;
use prettytable::row;
use prettytable::{Table, Row, Cell};
use indicatif::{ProgressBar, ProgressStyle};

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

  let mut all_stats = Vec::new();

  // Count total files to process for progress bar
  let total_files: usize = args.files.iter()
    .map(|input| {
      let path = Path::new(input);
      if path.is_dir() {
        // Count files in the directory
        fs::read_dir(path).unwrap().count()
      } else if path.is_file() {
        1 // Single file counts as one
      } else {
        0 // Not a valid file or directory
      }
    })
    .sum();

    let pb = ProgressBar::new(total_files as u64);
    let style = ProgressStyle::default_bar()
      .template("{spinner:.green} {msg} [{bar:40}] {percent:>3}% ({elapsed_precise})")
      .unwrap();
    pb.set_style(style);

  for input in args.files {
    let path = Path::new(&input);

    if path.is_file() {
      let file_stats = utils::process_file(path, &lang_map);
      all_stats.push(file_stats);
      pb.inc(1);
    } else if path.is_dir() {
      utils::process_directory(path, &lang_map, &pb, &mut all_stats);
    } else {
      println!("{} is neither a valid file not directory path", input);
    }
  }

  pb.finish_with_message("Processing complete. [100%]\n\n");

  // Display the collected stats in a table
  let mut table = Table::new();
  table.add_row(row!["File Name", "Language", "Total Lines", "Code Lines", "Blank Lines", "Comment Lines"]);

  for stats in all_stats {
    table.add_row(Row::new(vec![
      Cell::new(&stats.file_name),
      Cell::new(&stats.file_type),
      Cell::new(&stats.total_lines.to_string()),
      Cell::new(&stats.code_lines.to_string()),
      Cell::new(&stats.blank_lines.to_string()),
      Cell::new(&stats.comment_lines.to_string()),
    ]));
  }

  table.printstd(); // Print the table to standard output
}
