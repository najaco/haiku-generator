use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

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

fn build_syllable_table(words: &Vec<String>) -> HashMap<u32, Vec<String>> {
    let mut syllable_table = HashMap::<u32, Vec<String>>::new();
    for word in words {
        let syllable_count = count_syllables(&word);
        syllable_table
            .entry(syllable_count)
            .or_insert(Vec::new())
            .push(word.to_string());
    }
    return syllable_table;
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

#[derive(PartialEq)]
pub struct HaikuGenerator {
    syllable_table: HashMap<u32, Vec<String>>,
}
impl HaikuGenerator {
    pub fn new(words: Vec<String>) -> HaikuGenerator {
        return HaikuGenerator {
            syllable_table: build_syllable_table(&words),
        };
    }

    pub fn generate_haiku(&self) -> Vec<String> {
        let haiku: Vec<String> = vec![
            build_n_syllable_sentence(&self.syllable_table, 5),
            build_n_syllable_sentence(&self.syllable_table, 7),
            build_n_syllable_sentence(&self.syllable_table, 5),
        ];
        return haiku;
    }
}
