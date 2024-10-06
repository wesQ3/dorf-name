use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

mod word;
mod files;
pub use crate::files::*;

#[derive(Default)]
struct NamePreset {
    favor_symbols: Vec<String>,
    exclude_symbols: Vec<String>,
}

#[derive(Default)]
pub struct Language {
    words: HashMap<String, Word>,
    symbol_index: HashMap<String, Vec<String>>,
}

impl Language {
    pub fn word(&self, w: &str) -> Option<&Word> {
        self.words.get(w)
    }

    pub fn load() -> std::io::Result<Self> {
        // files must be utf8 converted
        let f = File::open("data/language_words.txt")?;
        let mut reader = BufReader::new(f);
        let mut words = files::read_lang_file(&mut reader)?;

        let f = File::open("data/language_SYM.txt")?;
        let mut reader = BufReader::new(f);
        let symbol_index = files::add_symbols(&mut reader, &mut words)?;

        let f = File::open("data/language_DWARF.txt")?;
        let mut reader = BufReader::new(f);
        let _ = files::add_translation(&mut reader, &mut words, "DWARF".to_string());
        Ok(Language { words, symbol_index })
    }

    pub fn npc_name(&self, preset_lang: &str) -> String {
        let preset = NamePreset {
            favor_symbols: ["ARTIFICE", "EARTH"]
                .iter().map(|s| s.to_string()).collect(),
            exclude_symbols: ["DOMESTIC", "SUBORDINATE", "EVIL", "FLOWERY", "NEGATIVE", "UGLY", "NEGATOR"]
                .iter().map(|s| s.to_string()).collect(),
        };

        // TODO cache name pool for preset, ran into ownership problems
        let keys = self.name_pool(preset);
        let (given, sur_1, sur_2) = Self::pick_name_words(keys);

        let given_dw = self.words.get(given.as_str()).unwrap()
            .translations.get(preset_lang).unwrap();
        let sur_1 = self.words.get(sur_1.as_str()).unwrap().pick_name();
        let sur_2 = self.words.get(sur_2.as_str()).unwrap().pick_name();
        format!("{} {}{}",
                ucfirst(given_dw),
                ucfirst(sur_1.to_lowercase().as_str()), sur_2.to_lowercase())
    }

    fn pick_name_words(pool: Vec<&String>) -> (String, String, String) {
        let mut rng = thread_rng();
        let given = pool.choose(&mut rng).unwrap().to_string();
        let sur_1 = pool.choose(&mut rng).unwrap().to_string();
        let sur_2 = pool.choose(&mut rng).unwrap().to_string();
        (given, sur_1, sur_2)
    }

    fn name_pool(&self, preset: NamePreset) -> Vec<&String> {
        let mut pool: Vec<&String> = vec![];
        let mut blacklist: Vec<&String> = vec![];
        for (symbol, s_words) in self.symbol_index.iter() {
            if symbol.starts_with("NAME_") {
                // name* symbols are for places/structures
                continue;
            }
            if preset.exclude_symbols.iter().any(|s| s == symbol) {
                blacklist.extend(s_words);
                continue;
            }
            if preset.favor_symbols.iter().any(|s| s == symbol) {
                // favored symbols get extra chances in the dice roll
                pool.extend(s_words);
                pool.extend(s_words);
            }
            pool.extend(s_words);
        }
        // sorted search is much faster than .contains()
        blacklist.sort();
        pool.retain(|&key| !blacklist.binary_search(&key).is_ok());
        // exclude prefixes
        pool.retain(|&key| self.words.get(key).unwrap().prefix.is_none());
        return pool
    }
}

// https://stackoverflow.com/a/38406885
fn ucfirst(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
