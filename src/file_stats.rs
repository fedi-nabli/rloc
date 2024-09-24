pub struct FileStats {
  pub file_name: String,
  pub file_type: String,
  pub total_lines: i64,
  pub code_lines: i64,
  pub blank_lines: i64,
  pub comment_lines: i64
}

impl FileStats {
  pub fn new(file_type: &'static str) -> Self {
    Self {
      file_name: "".to_string(),
      file_type: file_type.to_string(),
      total_lines: 0,
      code_lines: 0,
      blank_lines: 0,
      comment_lines: 0
    }
  }
}