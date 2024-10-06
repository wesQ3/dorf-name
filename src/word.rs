#![allow(dead_code)]
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Default)]
pub struct Word {
    pub root: String,
    pub noun: Option<Noun>,
    pub verb: Option<Verb>,
    pub adj: Option<Adjective>,
    pub prefix: Option<Prefix>,
    pub translations: HashMap<String, String>, // Language -> Translation
    pub symbols: Vec<String>,
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
    pub fn parse(line: &String, reader: &mut BufReader<File>) -> io::Result<Option<Word>> {
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
pub struct WordForm {
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
pub struct Noun {
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
pub struct Verb {
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
pub struct Adjective {
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
pub struct Prefix {
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


