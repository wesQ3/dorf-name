#![allow(dead_code)]
#![allow(unused)]
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

struct Word {
    root: String,
    noun: Option<Noun>,
    verb: Option<Verb>,
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
        // split out root
        // while let part = parse_word_form
        //    noun/verb/adj etc

        // placeholder return:
        Ok(Some(Word { root: "FOO".to_string(), noun: None, verb: None } ))
    }
}

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
