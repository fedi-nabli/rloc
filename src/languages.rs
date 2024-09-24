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

  // Web languages
  lang_map.insert("css", "CSS");
  lang_map.insert("php", "PHP");
  lang_map.insert("rb", "Ruby");
  lang_map.insert("html", "HTML");
  lang_map.insert("js", "JavaScript");
  lang_map.insert("ts", "TypeScript");
  lang_map.insert("erb", "Embedded Ruby");
  lang_map.insert("jsx", "JavaScript React");
  lang_map.insert("tsx", "TypeScript React");

  // Java JVM-based languages
  lang_map.insert("java", "Java");
  lang_map.insert("kt", "Kotlin");
  lang_map.insert("scala", "Scala");
  lang_map.insert("clj", "Clojure");
  lang_map.insert("groovy", "Groovy");

  // Shell languages
  lang_map.insert("zsh", "Zsh");
  lang_map.insert("bash", "Bash");
  lang_map.insert("bat", "Batch");
  lang_map.insert("ps1", "PowerShell");
  lang_map.insert("sh", "Shell Script");

  lang_map
}