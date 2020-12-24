use std::collections::BTreeMap;
use std::path::Path;

use serde::{Serialize, Deserialize};

use rand::Rng;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Kanji {
    on_readings : Vec<String>,
    kun_readings : Vec<String>,
    meaning : Vec<String>,
    literal : char,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Entry {
    kanji : Kanji,
    confidence_level : i32, // 0 - 5?
}

#[derive(Debug)]
struct Book {
    kanjis : BTreeMap <char, Entry>,
}

impl Book {
    pub fn new(kanjis : BTreeMap<char, Entry>) -> Self {
        Book {
            kanjis,
        }
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
    pub fn save(&self, file_name : &str) {
        println!("will now attempt to serialize: {:?}", self);

        let serialized = serde_json::to_string(&self.kanjis).expect("Unable to serialize book!");
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name).expect("Couldnt open file for writing.");

        f.write_all(serialized.as_bytes());
    }
}

mod kanji_dict;

fn main() {
    let file_name = "dict.json";
    let mut entries: BTreeMap<char, Entry>  = if Path::new(file_name).exists() {
        serde_json::from_reader(std::fs::File::open(file_name).expect("Couldnt open file")).expect(&format!("Unable to parse out dict from {}", file_name))
    } else {
        BTreeMap::new()
    };

    let dict : kanji_dict::KanjiDictionary = serde_xml_rs::from_reader(std::fs::File::open("kanjidic2.xml").expect("Couldnt open dict file")).expect("Couldnt load dict!");
    let mut lookup_dict = Vec::new();

    let mut book = Book::new(entries);

    for c in dict.character {
        let mut reading_on = Vec::new();
        let mut reading_kun = Vec::new();
        let mut meaning = Vec::new();
        if let Some(reading_meaning) = c.reading_meaning {
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

        lookup_dict.push(Kanji {
            meaning,
            literal: c.literal,
            kun_readings: reading_kun,
            on_readings: reading_on,
        });
    }

    let term = console::Term::stdout();

    loop {
        term.clear_screen().unwrap();

        // term.write_line(&format!("how many chars: {}", dict.character.len())).unwrap();
        // for c in &dict.character {
        //     term.write_line(&format!("c {:?}", c));
        //     term.read_key();
        // }


        term.write_line("Poll[y] add[a] add-[f]ull [l]ist anything else exits.").unwrap();
        match term.read_char().unwrap() {
            'y' => {

            }
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
                term.write_line("Gimme Pattern to search the dict by either kanji or meaning: ").unwrap();
                let pattern = term.read_line().unwrap();

                let mut matching_kanjis = Vec::new();

                for k in &lookup_dict {
                    if pattern.contains(k.literal) || k.meaning.contains(&pattern) {
                        matching_kanjis.push(k.clone());
                    }
                }

                term.write_line("Matched kanjis:");
                for (i, k) in matching_kanjis.iter().enumerate() {
                    term.write_line(&format!("{}: {:?}", i, k));
                }

                term.write_line("Has any of those matched your query? pick the number");

                let number : usize = term.read_line().unwrap().parse().expect("That wasnt a number");
                if let Some(k) = matching_kanjis.get(number) {
                    term.write_line(&format!("You have selected: {:?}", k));
                }

                term.write_line("Press return to continue.");
                term.read_line();



                // for k in &dict.character {
                    // if pattern.contains(k.literal) || k.reading_meaning.unwrap_or(kanji_dict::ReadingMeaning).rmgroup.meaning.iter().any(|m| m.iter().any(|m| m.value.contains(&pattern))) {
                    //     term.write_line(&format!("Found kanji matching your pattern: {}", k.literal));
                    //
                    //     term.write_line("Press key to continue");
                    //
                    //     term.read_key();
                    // }
                // }

                // term.write_line(&format!("processing kanji: {}", kanji));
                // let mut romanjis = Vec::new();
                // let mut meanings = Vec::new();
                //
                // term.write_line("Gimme its Romanji: ").unwrap();
                // let mut continue_prompting = true;
                // while continue_prompting {
                //     let romanji = term.read_line().unwrap();
                //     romanjis.push(romanji);
                //     term.write_line("More romanjis y/N ?");
                //
                //     continue_prompting = term.read_char().unwrap().to_ascii_lowercase() == 'y'
                // }
                //
                // term.write_line(&format!("Gimme meaning of({}): ", kanji)).unwrap();
                // let mut continue_prompting = true;
                // while continue_prompting {
                //     let meaning = term.read_line().unwrap();
                //     meanings.push(meaning);
                //     term.write_line("More Meanings y/N ?");
                //
                //     continue_prompting = term.read_char().unwrap().to_ascii_lowercase() == 'y'
                // }
                //
                // term.write_line(&format!("Kanji to be added: {}", kanji));
                // term.write_line(&format!("its romanjis are as follows: {:?}", romanjis));
                // term.write_line(&format!("its meanings are as follows: {:?}", meanings));
                // term.write_line("\nAdd it to the dictionary y/N?");
                //
                // if term.read_char().unwrap().to_ascii_lowercase() == 'y' {
                //     book.add(KanjiEntry {
                //         kanji,
                //         romanjis,
                //         confidence_level : Confidence { level : 0},
                //         meaning : meanings,
                //     });
                //
                //     book.save(file_name);
                // }
            }
            'l' => {
                // for (counter, (_, kanji)) in book.kanjis.iter().enumerate() {
                //     term.write_line(&format!("[{}] {}: {:?}", counter, kanji.kanji, kanji.romanjis));
                // }
                //
                // term.write_line("Press any key to continue.");
                // term.read_char();
            }
            _ => {
                return;
            }
        }
    }
}
