use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static UNIX_DICTIONARY: &'static str = "/usr/share/dict/words";

fn is_vowel(c: char) -> bool {
    return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
}
fn count_syllables(word: &String) -> u32 {
    let mut syllable_count: u32 = 0;
    let mut i = 0;
    while i < word.len() - 1 {
        if is_vowel(word.chars().nth(i).unwrap()) != is_vowel(word.chars().nth(i + 1).unwrap()) {
            syllable_count += 1;
            i += 1;
        }
        i += 1;
    }
    if word.chars().last() == Some('y') {
        syllable_count += 1;
    }
    return syllable_count;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn build_syllable_table() -> HashMap<u32, Vec<String>> {
    let mut book_reviews = HashMap::<u32, Vec<String>>::new();
    if let Ok(lines) = read_lines(UNIX_DICTIONARY) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map(|s| s.unwrap().to_lowercase()) {
            let syllable_count = count_syllables(&line);
            book_reviews
                .entry(syllable_count)
                .or_insert(Vec::new())
                .push(line);
        }
    }
    return book_reviews;
}

fn build_n_syllable_sentence(syllable_table: &HashMap<u32, Vec<String>>, mut n: u32) -> String {
    let mut rng = rand::thread_rng();
    let mut sentence: Vec<String> = Vec::new();
    while n > 0 {
        let word_syllable_length: u32 = rng.gen_range(1..(n + 1));
        n -= word_syllable_length;
        sentence.push(
            syllable_table[&word_syllable_length]
                .choose(&mut rng)
                .unwrap()
                .to_string(),
        );
    }
    return sentence.join(" ");
}

fn build_haiku(syllable_table: &HashMap<u32, Vec<String>>) -> Vec<String> {
    let haiku = vec![
        build_n_syllable_sentence(syllable_table, 5),
        build_n_syllable_sentence(syllable_table, 7),
        build_n_syllable_sentence(syllable_table, 5),
    ];
    return haiku;
}

fn main() {
    // let word: String = String::from("sore");
    let res = build_syllable_table();
    let haiku = build_haiku(&res);
    println!("{}", haiku.join("\n"))
}
