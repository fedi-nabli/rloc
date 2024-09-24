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

  // Assembly languages
  lang_map.insert("s", "Assembly");
  lang_map.insert("S", "Assembly");
  lang_map.insert("asm", "Assembly");

  // Scripting languages
  lang_map.insert("r", "R");
  lang_map.insert("lua", "Lua");
  lang_map.insert("pl", "Perl");
  lang_map.insert("py", "Python");
  lang_map.insert("pyx", "Cython");

  lang_map
}