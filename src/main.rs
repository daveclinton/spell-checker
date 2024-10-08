use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::io::{self, Write};

struct SpellChecker {
    words: HashMap<String, u32>,
    total_words: u32,
}

impl SpellChecker {
    fn new(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let re = Regex::new(r"\w+").unwrap();
        let mut words = HashMap::new();
        let mut total_words = 0;

        for line in reader.lines() {
            let line = line?;
            for word in re.find_iter(&line.to_lowercase()) {
                *words.entry(word.as_str().to_string()).or_insert(0) += 1;
                total_words += 1;
            }
        }
        Ok(SpellChecker { words, total_words })
    }
    
    fn p(&self, word: &str) -> f64 {
        *self.words.get(word).unwrap_or(&0) as f64 / self.total_words as f64
    }

    fn correction(&self, word: &str) -> String {
        self.candidates(word)
            .into_iter()
            .max_by(|a, b| self.p(a).partial_cmp(&self.p(b)).unwrap())
            .unwrap_or(word.to_string())
    }

    fn candidates(&self, word: &str) -> Vec<String> {
        let known_word = self.known(vec![word.to_string()]);
        if !known_word.is_empty() {
            return known_word;
        }
        let edit1 = self.known(self.edits1(word));
        if !edit1.is_empty() {
            return edit1;
        }
        let edit2 = self.known(self.edits1(word).iter().flat_map(|e1| self.edits1(e1)).collect());
        if !edit2.is_empty() {
            return edit2;
        }
        vec![word.to_string()]
    }

    fn known(&self, words: Vec<String>) -> Vec<String> {
        words.into_iter().filter(|w| self.words.contains_key(w)).collect()
    }

    fn edits1(&self, word: &str) -> Vec<String> {
        let splits: Vec<(&str, &str)> = (0..=word.len()).map(|i| word.split_at(i)).collect();
        let deletes: Vec<String> = splits
            .iter()
            .filter(|(_, r)| !r.is_empty())
            .map(|(l, r)| format!("{}{}", l, &r[1..]))
            .collect();
        let transposes: Vec<String> = splits
            .iter()
            .filter(|(_, r)| r.len() > 1)
            .map(|(l, r)| format!("{}{}{}{}", l, r.chars().nth(1).unwrap(), r.chars().next().unwrap(), &r[2..]))
            .collect();
        let replaces: Vec<String> = splits
            .iter()
            .filter(|(_, r)| !r.is_empty())
            .flat_map(|(l, r)| {
                ('a'..='z').map(move |c| format!("{}{}{}", l, c, &r[1..]))
            })
            .collect();
        let inserts: Vec<String> = splits
            .iter()
            .flat_map(|(l, r)| {
                ('a'..='z').map(move |c| format!("{}{}{}", l, c, r))
            })
            .collect();

        [deletes, transposes, replaces, inserts].concat()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting spell checker...");
    let checker = match SpellChecker::new("big.txt"){
        Ok(c)=> c,
        Err(e) => {
            eprintln!("Error creating a Spellchcker: {}", e);
            return Err(e);
        }
    };
    println!("Total words in dictrionary: {}", checker.total_words);

    let test_words = vec!["Spelling", "korrectud"];
    loop {
        print!("Enter a word to correct (or 'quit' to exit):");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let word = input.trim();

        if word.to_lowercase() == "quit"{
            break;
        }
    }
    Ok(())
}