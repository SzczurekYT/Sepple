use std::{
    cmp::{Reverse, max, min},
    fs,
};

use phonetics::confusion;
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

const DICTIONARY_PATH: &str = "dictionary.json";
const MAX_MISSING_CHARACTERS: usize = 3;
pub const DEFAULT_CONFUSION_DISTANCE_THRESHOLD: f64 = 0.15;

const DEBUG_ENABLED: bool = false;
macro_rules! debug_print {
    ($expr:expr) => {
        if DEBUG_ENABLED {
            println!($expr);
        }
    };
}

pub struct Dictionary {
    words: Vec<(String, usize)>,
    pub shortest_word_len: usize,
    pub longest_considered_word_len: usize,
    pub confusion_distance_threshold: f64,
}

impl Dictionary {
    pub fn load(confusion_distance_threshold: f64) -> Self {
        let text = fs::read_to_string(DICTIONARY_PATH)
            .unwrap_or_else(|_| panic!("Unable to read {DICTIONARY_PATH}"));
        let words: Vec<String> = serde_json::from_str(&text).unwrap();
        let mut shortest_word_len = usize::MAX;
        let mut longest_considered_word_len = 0;
        let mut words: Vec<(String, usize)> = words
            .into_iter()
            .map(|word| {
                let len = word.graphemes(true).count();
                shortest_word_len = min(shortest_word_len, len);
                longest_considered_word_len = max(longest_considered_word_len, len);
                (word, len)
            })
            .collect();

        words.sort_by_key(|(_, len)| Reverse(*len));

        longest_considered_word_len += MAX_MISSING_CHARACTERS;

        Dictionary {
            words,
            shortest_word_len,
            longest_considered_word_len,
            confusion_distance_threshold,
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
                debug_print!("remaining: {string}");
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
        debug_print!("Trying {input}");
        let mut lowest_distance = f64::MAX;
        let mut result = None;
        for (dict_word, len) in &self.words {
            let max_diff = max_lookahead(*len) as isize;
            for i in -2..=max_diff {
                let (mut last_index, grapheme) = input
                    .grapheme_indices(true)
                    .take((*len as isize + i).max(0) as usize)
                    .last()
                    .expect("at least one grapheme");
                last_index += grapheme.len();
                let fragment = &input[..last_index];
                debug_print!(r#"Is "{fragment}" a "{dict_word}"?"#);
                let Some(distance) =
                    strings_are_similiar(fragment, dict_word, self.confusion_distance_threshold)
                else {
                    debug_print!("Nope");
                    continue;
                };
                debug_print!("Yep, distance {distance:.2}");
                if distance < lowest_distance {
                    lowest_distance = distance;
                    result = Some((fragment, dict_word.as_ref()));
                }
            }
        }
        result
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::load(DEFAULT_CONFUSION_DISTANCE_THRESHOLD)
    }
}

/// Returns the difference in levenshtein distance if it is close enough
fn strings_are_similiar(string: &str, pattern: &str, threshold: f64) -> Option<f64> {
    let distance = 1.0 - confusion::similarity(string, pattern);

    (distance <= threshold).then_some(distance)
}

const fn max_lookahead(len: usize) -> usize {
    match len {
        0..4 => 0,
        4..6 => 1,
        6..9 => 2,
        _ => MAX_MISSING_CHARACTERS,
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
        const REMAINDER: &str = "l";
        const SEQUENCE_SPLIT: &[&str] = &[
            "prizim", "fɛra", "kɛjfida", //
            "prizim", "fɛra", "kɛjfida", //
            "prizim", "fɛra", "kɛjfida", //
            "prizim", "fɛra", "kɛjfida", //
            "prizim", "fɛra", "kɛjfida", //
        ];

        let dict = Dictionary::default();
        let (words, consumed) = dict.greedy_search(SEQUENCE);

        assert_eq!(&words, SEQUENCE_SPLIT);
        assert_eq!(&SEQUENCE[consumed..], REMAINDER);
    }

    #[test]
    fn test_greedy_split_prizim_fera_kejfida_2() {
        const SEQUENCE: &str =
            "pɾizimfɛɾakɛlfidapɾilifɛrakajfidarizɲfɛrakajʃinakɾitunfɛɾaːkaiɸpɾiːzinferaːkɛlihinæl";
        const REMAINDER: &str = "ːkɛlihinæl";
        const SEQUENCE_SPLIT: &[&str] = &[
            "prizim", "fɛra", "kɛjfida", //
            "prizim", "kɛjfida", //
            "fɛra", "kɛjfida", //
            "fɛra",    //
            "prizim", "fɛra", //
        ];

        let dict = Dictionary::default();
        let (words, consumed) = dict.greedy_search(SEQUENCE);

        assert_eq!(&words, SEQUENCE_SPLIT);
        assert_eq!(&SEQUENCE[consumed..], REMAINDER);
    }

    #[test]
    fn test_greedy_split_prizim() {
        const SEQUENCE: &str = "pɾiːzim";
        const REMAINDER: &str = "";
        const SEQUENCE_SPLIT: &[&str] = &["prizim"];

        let dict = Dictionary::default();
        let (words, consumed) = dict.greedy_search(SEQUENCE);

        assert_eq!(&words, SEQUENCE_SPLIT);
        assert_eq!(&SEQUENCE[consumed..], REMAINDER);
    }

    #[test]
    fn test_greedy_split_prilifera() {
        const SEQUENCE: &str = "pɾilifɛra";
        const REMAINDER: &str = "ɛra";
        // const SEQUENCE_SPLIT: &[&str] = &["prizim", "fɛra"];
        // pɾilif matchest the best to prizim, so it eats the f and cuts the second word
        const SEQUENCE_SPLIT: &[&str] = &["prizim"];

        let dict = Dictionary::default();
        let (words, consumed) = dict.greedy_search(SEQUENCE);

        assert_eq!(&words, SEQUENCE_SPLIT);
        assert_eq!(&SEQUENCE[consumed..], REMAINDER);
    }

    #[test]
    fn test_greedy_split_no_match() {
        const SEQUENCE: &str = "Y]+g4Ty}F({7H!8nrn2(1ZH[Y)A0SSg4}0tXy!)013Vz}6kjZW(Fg{bpGY+D:Z1/X&5UmJ4L+X2=r8ji[a)h,i7[n7Ny9";
        const REMAINDER: &str = SEQUENCE;
        const SEQUENCE_SPLIT: &[&str] = &[];

        let dict = Dictionary::default();
        let (words, consumed) = dict.greedy_search(SEQUENCE);

        assert_eq!(&words, SEQUENCE_SPLIT);
        assert_eq!(&SEQUENCE[consumed..], REMAINDER);
    }
}
