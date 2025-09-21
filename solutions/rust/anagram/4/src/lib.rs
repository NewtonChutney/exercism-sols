use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // unimplemented!(
    //     "For the '{word}' word find anagrams among the following words: {possible_anagrams:?}"
    // );
    let mut sorted_word = word.to_lowercase().chars().collect::<Vec<char>>();
    sorted_word.sort_unstable();

    possible_anagrams
        .iter()
        .filter(|_word| {
            let pa = _word.to_lowercase();
            if word.to_lowercase() == pa || word.len() != pa.len() {
                return false;
            }
            let mut possible_anagram_vec = pa.chars().collect::<Vec<char>>();
            possible_anagram_vec.sort_unstable();
            sorted_word == possible_anagram_vec
        })
        .cloned()
        .collect::<HashSet<&str>>()
}
