mod languages;

use std::path::Path;

fn main() {
  println!("Hello, world!");
  let lang_map = languages::get_languages_map();
  let path = Path::new("main.asm");

  if let Some(extension) = path.extension() {
    if let Some(ext_str) = extension.to_str() {
      println!("extension is {}", ext_str);
      println!("extension detected is {}", lang_map.get(ext_str).copied().unwrap());
    }
  }
}
