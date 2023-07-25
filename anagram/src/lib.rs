use itertools::Itertools;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let word = word.to_lowercase();
    let word_sorted = word.chars().sorted().collect::<String>();

    for possible_anagram in possible_anagrams {
        let possible_anagram_sorted = possible_anagram
            .to_lowercase()
            .chars()
            .sorted()
            .collect::<String>();
        if word_sorted == possible_anagram_sorted && word != possible_anagram.to_lowercase() {
            anagrams.insert(*possible_anagram);
        }
    }

    return anagrams;
}
