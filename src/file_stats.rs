struct FileStats {
  file_type: &'static str,
  total_lines: i64,
  code_lines: i64,
  blank_lines: i64,
  comment_lines: i64
}

impl FileStats {
  pub fn new(file_type: &'static str) -> Self {
    Self {
      file_type,
      total_lines: 0,
      code_lines: 0,
      blank_lines: 0,
      comment_lines: 0
    }
  }

  pub fn from(file_type: &'static str, total_lines: i64, code_lines: i64, blank_lines: i64, comment_lines: i64) -> Self {
    Self {
      file_type,
      total_lines,
      code_lines,
      blank_lines,
      comment_lines
    }
  }
}