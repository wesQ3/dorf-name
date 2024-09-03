use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file_name = "data/language_DWARF.txt"; // utf8 converted
    let f = File::open(file_name)?;
    let reader = BufReader::new(f);

    let mut tl_map = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        if let Some(word) = process_line(line.clone()) {
            tl_map.insert(word.0, word.1);
        } else {
            println!("skipped {}", line);
        }
    }
    println!("parsed {} words", tl_map.keys().len());
    Ok(())
}

fn process_line(line: String) -> Option<(String, String)> {
    let line = line.trim();
    if line.starts_with("[T_WORD:") {
        line.strip_prefix("[T_WORD:")?
            .strip_suffix("]")?
            .split_once(':')
            // convert to String to keep ownership
            .map(|(k, v)| (k.to_string(), v.to_string()))
    } else {
        None
    }
}
