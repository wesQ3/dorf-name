use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let file_name = "data/language_DWARF.txt"; // utf8 converted
    let f = File::open(file_name)?;
    let reader = BufReader::new(f);

    let mut tl_map = HashMap::new();
    for line in reader.lines() {
        match process_line(&line?) {
            Some(word) => tl_map.insert(word.0, word.1),
            None => println!("skipped line {line:?}"),
        }
    }
    Ok(())
}

fn process_line(line: &String) -> Option<(&str, &str)> {
   if line.starts_with("[T_WORD:") {
       line.strip_prefix("[T_WORD:")?.strip_suffix("]\r")?.split_once(':')
   } else {
       None
   }
}
