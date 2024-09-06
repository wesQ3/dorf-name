use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Word {
    lang_en: String,
    lang_dw: String,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}", self.lang_en, self.lang_dw)
    }
}

fn main() -> std::io::Result<()> {
    let file_name = "data/language_DWARF.txt"; // utf8 converted
    let f = File::open(file_name)?;
    let reader = BufReader::new(f);

    let mut word_list = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if let Some(word) = process_line(line.clone()) {
            let word = Word { lang_en: word.0, lang_dw: word.1 };
            word_list.push(word);
        } else {
            println!("skipped {}", line);
        }
    }
    println!("parsed {} words", word_list.len());
    println!("{}", word_list[1]);
    println!("{}", word_list[10]);
    println!("{}", word_list[100]);
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
