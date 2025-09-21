use std::{collections::HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // unimplemented!(
    //     "For the '{word}' word find anagrams among the following words: {possible_anagrams:?}"
    // );
    let mut anagrams = HashSet::new();

    for _word in possible_anagrams{
        if word.to_lowercase() != _word.to_lowercase() 
        && word.len() == _word.len() {
            let mut my_word_vec = word.to_lowercase().chars().collect::<Vec<char>>();
            let mut possible_anagram_vec = _word.to_lowercase().chars().collect::<Vec<char>>();
            // println!("{:?}", my_word_vec);
            my_word_vec.sort_unstable();
            // println!("{:?}", my_word_vec);
            possible_anagram_vec.sort_unstable();

            if my_word_vec == possible_anagram_vec{
                anagrams.insert(_word.to_owned());
            }
        }
    }
    anagrams
}
