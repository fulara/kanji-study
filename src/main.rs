use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use rand::Rng;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Kanji {
    on_readings: Vec<String>,
    kun_readings: Vec<String>,
    meaning: Vec<String>,
    literal: char,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Entry {
    kanji: Kanji,
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

        panic!("should always pick out a random kanji.")
    }

    pub fn add(&mut self, entry: Entry) {
        self.kanjis.insert(entry.kanji.literal, entry);
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

mod kanji_dict;

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

fn build_sled_db(sled_db: &sled::Tree, lookup_dict: &Vec<Kanji>) {
    println!(
        "now writing to sled: {}",
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("before")
            .as_secs()
    );
    for k in lookup_dict {
        let v = serde_json::to_string(&k).expect("unable to ser");
        sled_db.insert(k.literal.to_string(), v.as_str());
    }
    sled_db.flush();
    println!(
        "done writing to sled: {}",
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("after")
            .as_secs()
    );
}

fn load_sled_db_to_silly_vec(sled_db: &sled::Tree) -> Vec<Kanji> {
    let mut silly_vec = Vec::new();
    println!(
        "now converting from sled: {}",
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("before")
            .as_secs()
    );
    for e in sled_db.iter() {
        let e = e.unwrap();
        // let key= String::from_utf8(e.0.to_vec()).unwrap();
        let kanji = serde_json::from_slice(&e.1.to_vec()).unwrap();

        silly_vec.push(kanji);
    }
    println!(
        "finished converting from sled: {}",
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("before")
            .as_secs()
    );

    silly_vec
}

fn parse_dict() -> kanji_dict::KanjiDictionary {
    serde_xml_rs::from_reader(std::fs::File::open("kanjidic2.xml").expect("Couldnt open dict file"))
        .expect("Couldnt load dict!")
}

fn main() {
    // open::that("test.svg");
    let c = 0x2c as char;
    println!("kanji_{:0>5x} c: {}", '円' as u32, c);
    // ::std::process::Command::new(r#"C:\programming\rust\kanji-initiator\test.svg"#).status().unwrap();
    // println!("hai");
    ::std::thread::sleep_ms(100000);
    let file_name = "dict.json";
    let mut entries: BTreeMap<char, Entry> = if Path::new(file_name).exists() {
        serde_json::from_reader(std::fs::File::open(file_name).expect("Couldnt open file"))
            .expect(&format!("Unable to parse out dict from {}", file_name))
    } else {
        BTreeMap::new()
    };

    let db = sled::open("kanji.db").expect("opening db files");
    let lookup_dict = load_sled_db_to_silly_vec(&db);

    // let mut lookup_dict = Vec::new();

    let mut book = Book::new(entries);

    let term = console::Term::stdout();

    loop {
        term.clear_screen().unwrap();

        // term.write_line(&format!("how many chars: {}", dict.character.len())).unwrap();
        // for c in &dict.character {
        //     term.write_line(&format!("c {:?}", c));
        //     term.read_key();
        // }

        term.write_line("Poll[y] add[a] add-[f]ull [l]ist anything else exits.")
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

                let mut matching_kanjis = Vec::new();

                for k in &lookup_dict {
                    if pattern.contains(k.literal) || k.meaning.iter().any(|m| m.contains(&pattern)) {
                        matching_kanjis.push(k.clone());
                    }
                }

                term.write_line("Matched kanjis:");
                for (i, k) in matching_kanjis.iter().enumerate() {
                    term.write_line(&format!("{}: {:?}", i, k));
                }

                term.write_line("Has any of those matched your query? pick the number");

                let number: usize = term
                    .read_line()
                    .unwrap()
                    .parse()
                    .expect("That wasnt a number");
                if let Some(k) = matching_kanjis.get(number) {
                    term.write_line(&format!("You have selected: {:?}", k));

                    term.write_line("Do you wish to add it to your knowledge base? [y/N]");
                    if term.read_char().unwrap().to_ascii_lowercase() == 'y' {
                        book.add_save(Entry {
                            kanji : k.clone(),
                            confidence_level : 0,
                        }, file_name);
                        term.write_line(&format!("Added {} to your base", k.literal));
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
            _ => {
                return;
            }
        }
    }
}
