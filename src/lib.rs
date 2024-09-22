#![allow(dead_code)]
#![allow(unused)]
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Word {
    root: String,
    noun: Option<Noun>,
    verb: Option<Verb>,
    // usages: Vec<Usage>,
    // translations: HashMap<String, String>, // Language -> Translation
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
        if line == "" {
            // return None("unexpected end of word")
            return Ok(None)
        }
        if !line.starts_with("[WORD:") {
            // return None("not a word block")
            return Ok(None)
        }
        let trimmed = line.trim_end();
        let root = Self::parse_root(trimmed);
        println!("matched root {}", root);
        while let Ok(Some(part)) = WordForm::parse(reader) {
            println!("  form {:?}", part);
            // match part.form_type
            //  NOUN: Noun::from_form(part)
            //  VERB: Verb::from_form(part)
        }

        Ok(Some(Word { root, noun: None, verb: None } ))
    }

    fn parse_root(line: &str) -> String {
        line.trim_start_matches("[WORD:")
            .trim_end_matches("]")
            .to_string()
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
            return Ok(None)
        }
        // handle match
        let mut parts: Vec<&str> = header.trim()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(':')
            .filter(|s| !s.is_empty())
            .collect();
        println!("parts read: {:?}", parts);

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
                _ => { println!("! unknown usage what ho: {:?}", line) },
            }
            line.clear();
        }
        // rewind position to the line that didn't match
        let line_len: i64 = line.len().try_into().unwrap();
        reader.seek_relative(-line_len);

        Ok(Some(
            Self {
                form_type: parts.remove(0).to_string(),
                forms: parts.into_iter().map(|s| s.to_string()).collect(),
                usages,
            }
        ))
    }
}

#[derive(Debug)]
struct Noun {
    singular: String,
    plural: String,
    front_compound_sing: bool,
    rear_compound_sing: bool,
    the_compound_sing: bool,
    the_singular: bool,
    rear_compound_plural: bool,
    of_plural: bool,
}

#[derive(Debug)]
struct Verb {
    infinitive: String,
    third_person_sing: String,
    past_tense: String,
    past_participle: String,
    present_participle: String,
    standard_verb: bool,
}

pub struct Language {
    words: Vec<Word>,
}

impl Language {
    pub fn load() -> std::io::Result<Self> {
        let file_name = "data/language_words.txt"; // utf8 converted
        let f = File::open(file_name)?;
        let mut reader = BufReader::new(f);
        Ok(Language {
            words: Self::read_lang_file(&mut reader)?
        })
    }

    fn read_lang_file(reader: &mut BufReader<File>) -> std::io::Result<Vec<Word>> {
        let mut words = Vec::new();
        let mut line = String::new();
        while reader.read_line(&mut line)? != 0 {
            if let Ok(Some(word)) = Word::parse(&line, reader) {
                words.push(word)
            }
            line.clear();
        }
        Ok(words)
    }

    pub fn npc_name(&self) -> String {
        "Urist McNotImplementedYet".to_string()
    }
}
