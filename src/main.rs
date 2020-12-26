use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::kanji_strokes::KanjiDrawRecipe;
use bincode::deserialize;
use console::Term;
use rand::Rng;
use std::io::{BufReader, Write};

mod kanji_dict;
mod kanji_strokes;

#[derive(Serialize, Deserialize, Clone)]
struct Kanji {
    on_readings: Vec<String>,
    kun_readings: Vec<String>,
    meaning: Vec<String>,
    literal: char,
}

impl Kanji {
    fn pretty_print(&self) -> String {
        // maybe as display one day.
        format!(
            "kanji: {}, meanings: {:?}, on_readings: {:?}, kun_readings: {:?}",
            self.literal, self.meaning, self.on_readings, self.kun_readings
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Entry {
    kanji: char,
    confidence_level: i32, // 0 - 5?
}

#[derive(Debug)]
struct Book {
    kanjis: BTreeMap<char, Entry>,
}

impl Book {
    pub fn new(kanjis: BTreeMap<char, Entry>) -> Self {
        Book { kanjis }
    }

    pub fn roll(&self) -> Kanji {
        // let mut rng = rand::thread_rng();
        // let v= Rng::gen_range(&mut rng, 0..self.previous_sum);
        //
        // let mut stopper = 0;
        // for (_, k) in self.kanjis.iter() {
        //     stopper += k.confidence_level.val();
        //
        //     if stopper >= v {
        //         return k.clone();
        //     }
        // }

        panic!("should always pick out a random kanji. not really, go from lowest confidenc elevel and there pick now")
    }

    pub fn add(&mut self, entry: Entry) {
        self.kanjis.insert(entry.kanji, entry);
    }
    pub fn save(&self, file_name: &str) {
        println!("will now attempt to serialize: {:?}", self);

        let serialized = serde_json::to_string(&self.kanjis).expect("Unable to serialize book!");
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name)
            .expect("Couldnt open file for writing.");

        f.write_all(serialized.as_bytes());
    }

    pub fn add_save(&mut self, entry: Entry, file_name: &str) {
        self.add(entry);
        self.save(file_name);
    }
}

fn convert_parsed_to_kanji_vec(kanji_dictionary: &kanji_dict::KanjiDictionary) -> Vec<Kanji> {
    let mut kanji_vec = Vec::new();
    for c in &kanji_dictionary.character {
        let mut reading_on = Vec::new();
        let mut reading_kun = Vec::new();
        let mut meaning = Vec::new();
        if let Some(reading_meaning) = c.reading_meaning.clone() {
            let reading = reading_meaning.rmgroup.reading.unwrap_or(vec![]);
            for r in reading {
                if r.r_type == "ja_on" {
                    reading_on.push(r.value)
                } else if r.r_type == "ja_kun" {
                    reading_kun.push(r.value)
                }
            }

            let meaning_dict = reading_meaning.rmgroup.meaning.unwrap_or(vec![]);
            for m in meaning_dict {
                if m.m_lang == "en" {
                    meaning.push(m.value);
                }
            }
        }

        kanji_vec.push(Kanji {
            meaning,
            literal: c.literal,
            kun_readings: reading_kun,
            on_readings: reading_on,
        });
    }

    kanji_vec
}

fn dump_kanjis(lookup_dict: &Vec<Kanji>) {
    let v = serde_json::to_string(&lookup_dict).expect("unable to ser");
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("kanji.json")
        .expect("Couldnt open file showcase.svg for writing.");

    f.write(v.as_bytes());
}

fn dump_db(db: &Database) {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("db.bin")
        .expect("Couldnt open file showcase.svg for writing.");
    bincode::serialize_into(f, db);
}

fn load_db_from_plain_file() -> Database {
    let f = std::fs::OpenOptions::new()
        .read(true)
        .open("db.bin")
        .expect("couldnt load kanji.json for reading");
    let reader = BufReader::new(f);
    let db = bincode::deserialize_from(reader).expect("couldnt decode kanji vec");

    db
}

fn parse_dict() -> kanji_dict::KanjiDictionary {
    serde_xml_rs::from_reader(std::fs::File::open("kanjidic2.xml").expect("Couldnt open dict file"))
        .expect("Couldnt load dict!")
}

#[derive(Serialize, Deserialize, Clone)]
struct Database {
    kanjis: Vec<Kanji>,
    strokes: BTreeMap<char, kanji_strokes::KanjiDrawRecipe>,
}

impl Database {
    fn find(&self, pattern: &str) -> Vec<(Kanji, Option<KanjiDrawRecipe>)> {
        let mut matching_kanjis = Vec::new();

        for k in &self.kanjis {
            if pattern.contains(k.literal) || k.meaning.iter().any(|m| m == pattern) {
                matching_kanjis.push((k.clone(), self.strokes.get(&k.literal).cloned()));
            }
        }

        matching_kanjis
    }
}

fn ask_user_to_select_one_from_result(
    term: &console::Term,
    pattern: &str,
    results: &[(Kanji, Option<KanjiDrawRecipe>)],
) -> Option<(Kanji, Option<KanjiDrawRecipe>)> {
    if results.is_empty() {
        term.write_line(&format!(
            "Your pattern: '{}' has not matched any of the kanjis in db.",
            pattern
        ));
    }

    if results.len() == 1 {
        return Some(results[0].clone());
    }

    term.write_line("Matched kanjis:");
    for (i, k) in results.iter().enumerate() {
        term.write_line(&format!("{}: {}", i, k.0.pretty_print()));
    }

    term.write_line("Has any of those matched your query? pick the number");

    let number: usize = term
        .read_line()
        .unwrap()
        .parse()
        .expect("That wasnt a number");
    if let Some(k) = results.get(number) {
        return Some(k.clone());
    } else {
        term.write_line(&format!(
            "Number: {}, you have selected was out of range [{}-{}]",
            number,
            0,
            results.len() - 1
        ));
    }

    return None;
}

fn main() {
    let file_name = "dict.json";
    let mut entries: BTreeMap<char, Entry> = if Path::new(file_name).exists() {
        serde_json::from_reader(std::fs::File::open(file_name).expect("Couldnt open file"))
            .expect(&format!("Unable to parse out dict from {}", file_name))
    } else {
        BTreeMap::new()
    };

    if !std::path::Path::new("db.bin").exists() {
        let kanjivg = kanji_strokes::parse_kanjivg();
        let strokes = kanji_strokes::kanjivg_into_strokes(&kanjivg);
        let dict = parse_dict();

        let parsed = convert_parsed_to_kanji_vec(&dict);
        let db = Database {
            strokes: strokes.dict,
            kanjis: parsed,
        };

        dump_db(&db);
    }

    let db = load_db_from_plain_file();

    let mut book = Book::new(entries);

    let term = console::Term::stdout();

    loop {
        term.clear_screen().unwrap();

        // term.write_line(&format!("how many chars: {}", dict.character.len())).unwrap();
        // for c in &dict.character {
        //     term.write_line(&format!("c {:?}", c));
        //     term.read_key();
        // }

        term.write_line("Poll[y] add[a] add-[f]ull [l]ist anything else exits. [s]troke")
            .unwrap();
        match term.read_char().unwrap() {
            'y' => {}
            'f' => {
                // term.write_line("Selected add full, easier faster adding, here is a template:").unwrap();
                // term.write_line(r#"{"romanjis":["ichi"],"meaning":["one"],"kanji":"一","confidence_level":{"level":0}}"#).unwrap();
                // let full_entry = term.read_line().unwrap();
                // let deserialized = serde_json::from_str(&full_entry).expect(&format!("Couldnt deserialize your entry: {}", full_entry));
                // term.write_line(&format!("add? y/N: {:?}", deserialized)).unwrap();
                // if term.read_char().unwrap().to_ascii_lowercase() == 'y' {
                //     book.add(deserialized);
                //     term.write_line("Added, press any key to continue.").unwrap();
                //     term.read_key().unwrap();
                // }
            }
            'a' => {
                term.write_line("Gimme Pattern to search the dict by either kanji or meaning: ")
                    .unwrap();
                let pattern = term.read_line().unwrap();
                let matching_kanjis = db.find(&pattern);

                if let Some(single_result) =
                    ask_user_to_select_one_from_result(&term, &pattern, &matching_kanjis)
                {
                    term.write_line(&format!(
                        "You have selected: {}",
                        single_result.0.pretty_print()
                    ));

                    term.write_line("Do you wish to add it to your knowledge base? [y/N]");
                    if term.read_char().unwrap().to_ascii_lowercase() == 'y' {
                        book.add_save(
                            Entry {
                                kanji: single_result.0.literal,
                                confidence_level: 0,
                            },
                            file_name,
                        );
                        term.write_line(&format!("Added {} to your base", single_result.0.literal));
                    } else {
                        term.write_line("Skipping addition.");
                    }
                }

                term.write_line("Press return to continue.");
                term.read_line();
            }
            'l' => {
                for (counter, (_, kanji)) in book.kanjis.iter().enumerate() {
                    term.write_line(&format!("[{}] {:?}", counter, kanji.kanji));
                }

                term.write_line("Press any key to continue.");
                term.read_char();
            }
            's' => {
                term.write_line("Type in pattern by which you want to search");
                let pattern = term.read_line().expect("char was supposed to be here!");
                let result = db.find(&pattern);
                if result.is_empty() {
                    term.write_line(&format!(
                        "pattern you've put in: {} does not exist in db.",
                        pattern
                    ));
                    continue;
                }
                //meh we need to handle multiple prints but for now lets just take first one.

                if let Some(single_result) =
                    ask_user_to_select_one_from_result(&term, &pattern, &result)
                {
                    let strokes = &single_result.1;
                    if let Some(strokes) = strokes {
                        let body = strokes.generate_svg();
                        let mut f = std::fs::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open("showcase.svg")
                            .expect("Couldnt open file showcase.svg for writing.");

                        write!(f, "{}", body);
                        open::that("showcase.svg");
                    } else {
                        term.write_line(&format!(
                            "Kanji has been recognized but it seems we dont have strokes for it: {}",
                            single_result.0.literal
                        ));
                    };
                }
                term.write_line("Press any key to continue.");
                term.read_char();
                continue;
            }
            _ => {
                return;
            }
        }
    }
}
