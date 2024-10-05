#![allow(dead_code)]
#![allow(unused)]
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Default)]
pub struct Word {
    root: String,
    noun: Option<Noun>,
    verb: Option<Verb>,
    adj: Option<Adjective>,
    prefix: Option<Prefix>,
    translations: HashMap<String, String>, // Language -> Translation
    symbols: Vec<String>,
}

#[derive(Debug)]
enum Usage {
    AdjDist1,
    AdjDist2,
    AdjDist3,
    AdjDist4,
    AdjDist5,
    AdjDist6,
    AdjDist7,
    FrontCompoundAdj,
    FrontCompoundNounPlur,
    FrontCompoundNounSing,
    FrontCompoundPrefix,
    OfNounPlur,
    OfNounSing,
    RearCompoundAdj,
    RearCompoundNounPlur,
    RearCompoundNounSing,
    StandardVerb,
    TheCompoundAdj,
    TheCompoundNounPlur,
    TheCompoundNounSing,
    TheCompoundPrefix,
    TheNounPlur,
    TheNounSing,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}

impl Word {
    fn parse(line: &String, reader: &mut BufReader<File>) -> io::Result<Option<Word>> {
        if line.trim() == "" {
            // println!("unexpected end of word");
            return Ok(None);
        }
        if !line.starts_with("[WORD:") {
            let ignore = [
                "language_words",
                "[OBJECT:LANGUAGE]",
            ];
            if ignore.iter().any(|s| line.trim() == *s) {
                return Ok(None);
            }
            println!("not a word block\n{}", line.trim());
            return Ok(None);
        }
        let trimmed = line.trim_end();
        let root = Self::parse_root(trimmed);
        // println!("matched root {}", root);
        let mut word = Word {
            root,
            ..Default::default()
        };
        while let Ok(Some(part)) = WordForm::parse(reader) {
            // println!("  form {:?}", part);
            if part.form_type == "NOUN" {
                word.noun = Some(Noun::from_form(part));
            } else if part.form_type == "VERB" {
                word.verb = Some(Verb::from_form(part));
            } else if part.form_type == "ADJ" {
                word.adj = Some(Adjective::from_form(part));
            } else if part.form_type == "PREFIX" {
                word.prefix = Some(Prefix::from_form(part));
            }
        }

        Ok(Some(word))
    }

    fn parse_root(line: &str) -> String {
        line.trim_start_matches("[WORD:")
            .trim_end_matches("]")
            .to_string()
    }

    pub fn pick_name(&self) -> String {
        let mut pool = Vec::new();
        if let Some(noun) = &self.noun {
            pool.push(noun.singular.clone());
        }
        if let Some(verb) = &self.verb {
            pool.push(verb.infinitive.clone());
            pool.push(verb.past_tense.clone());
            pool.push(verb.past_participle.clone());
            pool.push(verb.present_participle.clone());
        }
        if let Some(adj) = &self.adj {
            pool.push(adj.adj.clone());
        }
        let mut rng = thread_rng();
        pool.choose(&mut rng).unwrap().to_string()
    }
}

#[derive(Debug)]
struct WordForm {
    form_type: String,
    forms: Vec<String>,
    usages: Vec<Usage>,
}

impl WordForm {
    fn parse(reader: &mut BufReader<File>) -> io::Result<Option<Self>> {
        let mut header = String::new();
        reader.read_line(&mut header)?;
        if !header.starts_with("\t[") {
            return Ok(None);
        }
        // handle match
        let mut parts: Vec<&str> = header
            .trim()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(':')
            .filter(|s| !s.is_empty())
            .collect();
        // println!("parts read: {:?}", parts);

        // consume remaining usage lines
        let mut line = String::new();
        let mut usages = Vec::new();
        while reader.read_line(&mut line)? != 0 && line.starts_with("\t\t") {
            match line.trim() {
                "[ADJ_DIST:1]" => usages.push(Usage::AdjDist1),
                "[ADJ_DIST:2]" => usages.push(Usage::AdjDist2),
                "[ADJ_DIST:3]" => usages.push(Usage::AdjDist3),
                "[ADJ_DIST:4]" => usages.push(Usage::AdjDist4),
                "[ADJ_DIST:5]" => usages.push(Usage::AdjDist5),
                "[ADJ_DIST:6]" => usages.push(Usage::AdjDist6),
                "[ADJ_DIST:7]" => usages.push(Usage::AdjDist7),
                "[FRONT_COMPOUND_ADJ]" => usages.push(Usage::FrontCompoundAdj),
                "[FRONT_COMPOUND_NOUN_PLUR]" => usages.push(Usage::FrontCompoundNounPlur),
                "[FRONT_COMPOUND_NOUN_SING]" => usages.push(Usage::FrontCompoundNounSing),
                "[FRONT_COMPOUND_PREFIX]" => usages.push(Usage::FrontCompoundPrefix),
                "[OF_NOUN_PLUR]" => usages.push(Usage::OfNounPlur),
                "[OF_NOUN_SING]" => usages.push(Usage::OfNounSing),
                "[REAR_COMPOUND_ADJ]" => usages.push(Usage::RearCompoundAdj),
                "[REAR_COMPOUND_NOUN_PLUR]" => usages.push(Usage::RearCompoundNounPlur),
                "[REAR_COMPOUND_NOUN_SING]" => usages.push(Usage::RearCompoundNounSing),
                "[STANDARD_VERB]" => usages.push(Usage::StandardVerb),
                "[THE_COMPOUND_ADJ]" => usages.push(Usage::TheCompoundAdj),
                "[THE_COMPOUND_NOUN_PLUR]" => usages.push(Usage::TheCompoundNounPlur),
                "[THE_COMPOUND_NOUN_SING]" => usages.push(Usage::TheCompoundNounSing),
                "[THE_COMPOUND_PREFIX]" => usages.push(Usage::TheCompoundPrefix),
                "[THE_NOUN_PLUR]" => usages.push(Usage::TheNounPlur),
                "[THE_NOUN_SING]" => usages.push(Usage::TheNounSing),
                _ => {
                    println!("! unknown usage what ho: {:?}", line)
                }
            }
            line.clear();
        }
        // rewind position to the line that didn't match
        let line_len: i64 = line.len().try_into().unwrap();
        let _ = reader.seek_relative(-line_len);

        Ok(Some(Self {
            form_type: parts.remove(0).to_string(),
            forms: parts.into_iter().map(|s| s.to_string()).collect(),
            usages,
        }))
    }
}

#[derive(Debug)]
struct Noun {
    singular: String,
    plural: String,
    usages: Vec<Usage>,
}

impl Noun {
    pub fn from_form(form: WordForm) -> Self {
        Self {
            singular: form.forms[0].clone(),
            plural: match form.forms.get(1) {
                Some(plural) => plural.clone(),
                None => "ERR_NO_PLURAL".to_string(),
            },
            usages: form.usages,
        }
    }
}

#[derive(Debug)]
struct Verb {
    infinitive: String,
    third_person_sing: String,
    past_tense: String,
    past_participle: String,
    present_participle: String,
    usages: Vec<Usage>,
}

impl Verb {
    pub fn from_form(form: WordForm) -> Self {
        Self {
            infinitive: form.forms[0].clone(),
            third_person_sing: form.forms[1].clone(),
            past_tense: form.forms[2].clone(),
            past_participle: form.forms[3].clone(),
            present_participle: form.forms[4].clone(),
            usages: form.usages,
        }
    }
}

#[derive(Debug)]
struct Adjective {
    adj: String,
    usages: Vec<Usage>,
}

impl Adjective {
    pub fn from_form(form: WordForm) -> Self {
        Self {
            adj: form.forms[0].clone(),
            usages: form.usages,
        }
    }
}

#[derive(Debug)]
struct Prefix {
    prefix: String,
    usages: Vec<Usage>,
}

impl Prefix {
    pub fn from_form(form: WordForm) -> Self {
        Self {
            prefix: form.forms[0].clone(),
            usages: form.usages,
        }
    }
}

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
        let mut words = Self::read_lang_file(&mut reader)?;

        let f = File::open("data/language_SYM.txt")?;
        let mut reader = BufReader::new(f);
        let symbol_index = Self::add_symbols(&mut reader, &mut words)?;

        let f = File::open("data/language_DWARF.txt")?;
        let mut reader = BufReader::new(f);
        let _ = Self::add_translation(&mut reader, &mut words, "DWARF".to_string());
        Ok(Language { words, symbol_index })
    }

    fn read_lang_file(reader: &mut BufReader<File>) -> std::io::Result<HashMap<String, Word>> {
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

    fn add_translation(
        reader: &mut BufReader<File>,
        words: &mut HashMap<String, Word>,
        tl_key: String,
    ) -> std::io::Result<()> {
        let mut line = String::new();
        while reader.read_line(&mut line)? != 0 {
            if let Some((k, v)) = Self::parse_translation_line(line.clone()) {
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

    fn add_symbols(
        reader: &mut BufReader<File>,
        words: &mut HashMap<String, Word>,
    ) -> std::io::Result<HashMap<String, Vec<String>>> {
        let mut symbols = HashMap::new();
        let mut line = String::new();
        while reader.read_line(&mut line)? != 0 {
            if let Ok(Some((symbol, list))) = Self::read_symbol_block(&line, reader, words) {
                // println!("symbol read {}", symbol);
                // println!("symbol index {:?}", list);
                symbols.insert(symbol.clone(), list);
            }
            line.clear();
        }
        Ok(symbols)
    }

    pub fn read_symbol_line(reader: &mut BufReader<File>) -> io::Result<Option<(String)>> {
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
        let s_root = Self::parse_symbol_root(trimmed);
        // println!("symbol s_root {}", s_root);

        let mut symbol_list = Vec::new();
        while let Ok(Some(s_word)) = Self::read_symbol_line(reader) {
            // println!("  s_word {} -> {}", s_root, s_word);
            words.entry(s_word.clone()).and_modify(|word| {
                word.symbols.push(s_root.clone());
            });
            symbol_list.push(s_word);
        }

        Ok(Some((s_root, symbol_list)))
    }


    pub fn npc_name(&self, preset_lang: &str) -> String {
        let preset = NamePreset {
            favor_symbols: ["ARTIFICE", "EARTH"]
                .iter().map(|s| s.to_string()).collect(),
            exclude_symbols: ["DOMESTIC", "SUBORDINATE", "EVIL", "FLOWERY", "NEGATIVE", "UGLY", "NEGATOR"]
                .iter().map(|s| s.to_string()).collect(),
        };

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
            if (symbol.starts_with("NAME_")) {
                // name* symbols are for places/structures
                continue;
            }
            if (preset.exclude_symbols.iter().any(|s| s == symbol)) {
                blacklist.extend(s_words);
                continue;
            }
            if (preset.favor_symbols.iter().any(|s| s == symbol)) {
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
