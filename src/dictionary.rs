use std::{
    cmp::{max, min},
    fs,
};

use strsim::generic_levenshtein;
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

const DICTIONARY_PATH: &str = "dictionary.json";

pub struct Dictionary {
    words: Vec<(String, usize)>,
    pub shortest_word_len: usize,
    pub longest_considered_word_len: usize,
}

impl Dictionary {
    pub fn load() -> Self {
        let text = fs::read_to_string(DICTIONARY_PATH)
            .unwrap_or_else(|_| panic!("Unable to read {DICTIONARY_PATH}"));
        let words: Vec<String> = serde_json::from_str(&text).unwrap();
        let mut shortest_word_len = usize::MAX;
        let mut longest_considered_word_len = 0;
        let words: Vec<(String, usize)> = words
            .into_iter()
            .map(|word| {
                let len = word.graphemes(true).count();
                shortest_word_len = min(shortest_word_len, len);
                longest_considered_word_len = max(longest_considered_word_len, len);
                (word, len)
            })
            .collect();
        longest_considered_word_len =
            longest_considered_word_len + max_difference(longest_considered_word_len);
        Dictionary {
            words,
            shortest_word_len,
            longest_considered_word_len,
        }
    }

    pub fn greedy_search(&self, mut string: &str) -> (Vec<&str>, usize) {
        let mut result = vec![];
        let mut offset = 0;
        let mut consumed = 0;
        // String has at least shortest_word_len graphemes
        while string
            .graphemes(true)
            .nth(self.shortest_word_len - 1)
            .is_some()
        {
            let word = self.try_find_word(string);
            if let Some((word, pattern)) = word {
                offset += word.len();
                consumed = offset;
                string = &string[word.len()..];
                result.push(pattern);
            } else {
                let first_grapheme_size = string
                    .graphemes(true)
                    .next()
                    .expect("at least one grapheme")
                    .len();
                offset += first_grapheme_size;
                string = &string[first_grapheme_size..];
            }
        }
        (result, consumed)
    }

    pub fn try_find_word<'dict, 'input>(
        &'dict self,
        input: &'input str,
    ) -> Option<(&'input str, &'dict str)> {
        let mut lowest_distance = usize::MAX;
        let mut result = None;
        for (dict_word, len) in &self.words {
            let max_diff = max_difference(*len);
            for i in 0..=max_diff {
                let (mut last_index, grapheme) = input
                    .grapheme_indices(true)
                    .take(*len + i)
                    .last()
                    .expect("at least one grapheme");
                last_index += grapheme.len();
                let fragment = &input[..last_index];
                let Some(distance) = strings_are_similiar(fragment, dict_word) else {
                    continue;
                };
                if distance < lowest_distance {
                    lowest_distance = distance;
                    result = Some((fragment, dict_word.as_ref()));
                }
            }
        }
        result
    }
}

/// Returns the difference in levenshtein distance if it is close enough
fn strings_are_similiar(string: &str, pattern: &str) -> Option<usize> {
    let len = string.graphemes(true).count();
    let max_difference = max_difference(len);
    let distance = generic_levenshtein(
        &GraphemesStringIterator(string),
        &GraphemesStringIterator(pattern),
    );
    (distance <= max_difference).then_some(distance)
}

const fn max_difference(len: usize) -> usize {
    match len {
        0..4 => 0,
        4..6 => 1,
        6..9 => 2,
        _ => 3,
    }
}

struct GraphemesStringIterator<'a>(&'a str);

impl<'b> IntoIterator for &GraphemesStringIterator<'b> {
    type Item = &'b str;
    type IntoIter = Graphemes<'b>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.graphemes(true)
    }
}

#[cfg(test)]
mod test {
    use crate::dictionary::Dictionary;

    #[test]
    fn test_greedy_split_prizim_fera_kejfida() {
        const SEQUENCE: &str = "prizimfɛrakɛlfidapriziɲfɛrakɛjfidariziɲfɛrakɛjfidapɾizinfɛraːkaifiːdaːpriːzɨjimfɛraːkɛifiːdɛl";
        const REMAINDER: &str = "ːkɛifiːdɛl";
        const SEQUENCE_SPLIT: &[&str] = &[
            "prizim", "fɛra", "kɛjfida", //
            "prizim", "fɛra", "kɛjfida", //
            "fɛra", "kɛjfida", //
            "prizim", "fɛra", //
            "fɛra",
        ];

        let dict = Dictionary::load();
        let (words, consumed) = dict.greedy_search(SEQUENCE);

        assert_eq!(&words, SEQUENCE_SPLIT);
        assert_eq!(&SEQUENCE[consumed..], REMAINDER);
    }

    #[test]
    fn test_greedy_split_prizim() {
        const SEQUENCE: &str = "pɾiːzim";
        const REMAINDER: &str = "";
        const SEQUENCE_SPLIT: &[&str] = &["prizim"];

        let dict = Dictionary::load();
        let (words, consumed) = dict.greedy_search(SEQUENCE);

        assert_eq!(&words, SEQUENCE_SPLIT);
        assert_eq!(&SEQUENCE[consumed..], REMAINDER);
    }

    #[test]
    fn test_greedy_split_no_match() {
        const SEQUENCE: &str = "Y]+g4Ty}F({7H!8nrn2(1ZH[Y)A0SSg4}0tXy!)013Vz}6kjZW(Fg{bpGY+D:Z1/X&5UmJ4L+X2=r8ji[a)h,i7[n7Ny9";
        const REMAINDER: &str = SEQUENCE;
        const SEQUENCE_SPLIT: &[&str] = &[];

        let dict = Dictionary::load();
        let (words, consumed) = dict.greedy_search(SEQUENCE);

        assert_eq!(&words, SEQUENCE_SPLIT);
        assert_eq!(&SEQUENCE[consumed..], REMAINDER);
    }
}
