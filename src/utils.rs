use crate::file_stats::FileStats;

use regex::Regex;
use std::path::Path;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

// Process a single file
pub fn process_file(path: &Path, lang_map: &std::collections::HashMap<&'static str, &'static str>) {
  if let Some(extension) = path.extension() {
    if let Some(ext_str) = extension.to_str() {
      let language = lang_map.get(ext_str).copied().unwrap_or("Unknown");
      println!("Processing file: {:?}, detected language: {}", path.display(), language);

      let mut stats = count_lines(path);
      stats.file_type = language;

      println!("Total lines: {}", stats.total_lines);
      println!("Code lines: {}", stats.code_lines);
      println!("Blank lines: {}", stats.blank_lines);
      println!("Comment lines: {}", stats.comment_lines);
    }
  }
}

// Recursively process all files in a directory
pub fn process_directory(dir: &Path, lang_map: &std::collections::HashMap<&'static str, &'static str>) {
  let entries = fs::read_dir(dir).expect("Unable to read directory");

  for entry in entries {
    let entry = entry.expect("Unable to get directory entry");
    let path = entry.path();

    if path.is_file() {
      process_file(&path, lang_map);
    } else if path.is_dir() {
      // Recursively process subdirectories
      process_directory(&path, lang_map);
    }
  }
}

fn count_lines(file_path: &Path) -> FileStats {
  let file = File::open(file_path).expect("Unable to open file");
  let reader = BufReader::new(file);

  let mut stats = FileStats::new("unknown");

  // Regex for matching single-line and multi-line comments
  let single_line_comment = Regex::new(r"^\s*(//|#)").unwrap(); // Matches // or #
  let multi_line_start = Regex::new(r"/\*").unwrap();           // Matches /*
  let multi_line_end = Regex::new(r"\*/").unwrap();             // Matches */

  let mut in_multiline_comment = false;

  for line in reader.lines() {
    let line = line.unwrap();
    stats.total_lines += 1;

    let trimmed = line.trim();

    if trimmed.is_empty() {
      stats.blank_lines += 1;
    } else if in_multiline_comment {
      // Continue multi-line comment block
      stats.comment_lines += 1;
      if multi_line_end.is_match(trimmed) {
        in_multiline_comment = false;
      }
    } else if multi_line_start.is_match(trimmed) {
      // Start of multi-line comment block
      stats.comment_lines += 1;
      if !multi_line_end.is_match(trimmed) {
        in_multiline_comment = true; // Detect ongoing multiline comment block
      }
    } else if single_line_comment.is_match(trimmed) {
      // Single-line comment
      stats.comment_lines += 1;
    } else {
      // Count as code if it's not a blank line or a comment
      stats.code_lines += 1;
    }
  }

  stats
}
