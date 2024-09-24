pub struct FileStats {
  pub file_type: &'static str,
  pub total_lines: i64,
  pub code_lines: i64,
  pub blank_lines: i64,
  pub comment_lines: i64
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
}