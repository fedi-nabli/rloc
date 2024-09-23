use std::collections::HashMap;

pub fn get_languages_map() -> HashMap<&'static str, &'static str> {
  let mut lang_map = HashMap::new();

  // System Languages
  lang_map.insert("c", "C");
  lang_map.insert("go", "Go");
  lang_map.insert("cs", "C#");
  lang_map.insert("cc", "C++");
  lang_map.insert("cpp", "C++");
  lang_map.insert("rs", "Rust");
  lang_map.insert("swift", "Swift");
  lang_map.insert("m", "Objective-C");
  lang_map.insert("mm", "Objective-C++");
  lang_map.insert("h", "C/C++ Header File");
  lang_map.insert("hpp", "C++ Header File");

  lang_map
}