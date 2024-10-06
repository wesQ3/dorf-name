use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub use crate::word::*;

pub fn read_lang_file(reader: &mut BufReader<File>) -> std::io::Result<HashMap<String, Word>> {
    let mut words = HashMap::new();
    let mut line = String::new();
    while reader.read_line(&mut line)? != 0 {
        if let Ok(Some(word)) = Word::parse(&line, reader) {
            words.insert(word.root.clone(), word);
        }
        line.clear();
    }
    Ok(words)
}

pub fn add_translation(
    reader: &mut BufReader<File>,
    words: &mut HashMap<String, Word>,
    tl_key: String,
) -> std::io::Result<()> {
    let mut line = String::new();
    while reader.read_line(&mut line)? != 0 {
        if let Some((k, v)) = parse_translation_line(line.clone()) {
            words.entry(k).and_modify(|word| {
                word.translations.insert(tl_key.clone(), v);
            });
        }
        line.clear();
    }
    Ok(())
}

pub fn parse_translation_line(line: String) -> Option<(String, String)> {
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

pub fn add_symbols(
    reader: &mut BufReader<File>,
    words: &mut HashMap<String, Word>,
) -> std::io::Result<HashMap<String, Vec<String>>> {
    let mut symbols = HashMap::new();
    let mut line = String::new();
    while reader.read_line(&mut line)? != 0 {
        if let Ok(Some((symbol, list))) = read_symbol_block(&line, reader, words) {
            // println!("symbol read {}", symbol);
            // println!("symbol index {:?}", list);
            symbols.insert(symbol.clone(), list);
        }
        line.clear();
    }
    Ok(symbols)
}

fn read_symbol_line(reader: &mut BufReader<File>) -> io::Result<Option<String>> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    if !line.starts_with("\t[") {
        return Ok(None);
    }
    let line = line.trim();
    let line = line.trim_start_matches("[S_WORD:")
        .trim_end_matches("]")
        .to_string();
    Ok(Some(line))
}

fn parse_symbol_root(line: &str) -> String {
    line.trim_start_matches("[SYMBOL:")
        .trim_end_matches("]")
        .to_string()
}

fn read_symbol_block(
    line: &String,
    reader: &mut BufReader<File>,
    words: &mut HashMap<String,Word>
) -> io::Result<Option<(String, Vec<String>)>> {
    if line.trim() == "" {
        // println!("unexpected end of symbol");
        return Ok(None);
    }
    if !line.starts_with("[SYMBOL:") {
        let ignore = [
            "language_SYM",
            "[OBJECT:LANGUAGE]"
        ];
        if ignore.iter().any(|s| line.trim() == *s) {
            return Ok(None);
        }
        println!("not a symbol block\n{}", line.trim());
        return Ok(None);
    }
    let trimmed = line.trim_end();
    let s_root = parse_symbol_root(trimmed);
    // println!("symbol s_root {}", s_root);

    let mut symbol_list = Vec::new();
    while let Ok(Some(s_word)) = read_symbol_line(reader) {
        // println!("  s_word {} -> {}", s_root, s_word);
        words.entry(s_word.clone()).and_modify(|word| {
            word.symbols.push(s_root.clone());
        });
        symbol_list.push(s_word);
    }

    Ok(Some((s_root, symbol_list)))
}
