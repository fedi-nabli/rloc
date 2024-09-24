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

  // Data and configuration files
  lang_map.insert("xml", "XML");
  lang_map.insert("ini", "INI");
  lang_map.insert("yml", "YAML");
  lang_map.insert("yaml", "YAML");
  lang_map.insert("json", "JSON");
  lang_map.insert("toml", "TOML");

  // Functional languages
  lang_map.insert("fs", "F#");
  lang_map.insert("ml", "OCaml");
  lang_map.insert("lisp", "Lisp");
  lang_map.insert("rkt", "Racket");
  lang_map.insert("hs", "Haskell");

  // Database Languages
  lang_map.insert("sql", "SQL");
  lang_map.insert("db", "Database File");

  // Markup languages
  lang_map.insert("tex", "LaTeX");
  lang_map.insert("md", "Markdown");
  lang_map.insert("rst", "reStructuredText");

  lang_map
}