use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    let word_lower = word.to_lowercase();
    let sorted_word = sort_letters(&word_lower);

    for &candidate in possible_anagrams {
        let candidate_lower = candidate.to_lowercase();

        if candidate_lower == word_lower {
            continue; // skip identical word
        }

        if sort_letters(&candidate_lower) == sorted_word {
            result.insert(candidate);
        }
    }

    result
}

fn sort_letters(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars.iter().collect()
}
