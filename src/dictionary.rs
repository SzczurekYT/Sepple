use std::{
    cmp::{Reverse, max, min},
    fs,
};

use phonetics::confusion;
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

const MAX_MISSING_CHARACTERS: usize = 3;

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
}

impl Dictionary {
    pub fn from_file(path: &str) -> Self {
        let text = fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to read {path}"));
        let words: Vec<String> = serde_json::from_str(&text).unwrap();
        Self::from_vec(words)
    }

    pub fn from_vec(words: Vec<String>) -> Self {
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
        }
    }

    pub fn find_words_in_string<'dict, 'input, 'out>(
        &'dict self,
        input: &'input str,
    ) -> (Vec<&'out str>, usize)
    where
        'dict: 'out,
        'input: 'out,
    {
        if input.is_empty() {
            return (vec![], 0);
        }

        let exact_search_entries = self.exact_find_words(input);

        let entry_count = exact_search_entries.len();
        let mut words = Vec::with_capacity(entry_count);

        let mut iter = exact_search_entries.into_iter();

        // Last one is handled separately
        for entry in iter.by_ref().take(entry_count - 1) {
            match entry {
                SearchEntry::Match(word) => words.push(word),
                SearchEntry::NoMatch(string) => {
                    let (found, _) = self.fuzzy_find_words(string);
                    words.extend(found);
                }
            }
        }

        let consumed;

        match iter.next().expect("unreachable for non empty input") {
            SearchEntry::Match(word) => {
                consumed = input.len();
                words.push(word);
            }
            SearchEntry::NoMatch(string) => {
                let (found, consumed_from_input) = self.fuzzy_find_words(string);
                words.extend(found);
                consumed = input.len() - string.len() + consumed_from_input;
            }
        }

        (words, consumed)
    }

    pub fn exact_find_words<'a>(&'a self, mut string: &'a str) -> Vec<SearchEntry<'a>> {
        use SearchEntry::*;
        let mut found_words = vec![];

        let mut i = 0;

        'outer: while i < string.len() {
            for (pattern, _) in &self.words {
                let end_index = i + pattern.len();
                if !string.is_char_boundary(end_index) {
                    continue;
                }
                let fragment = &string[i..end_index];
                if fragment == pattern {
                    let before = &string[..i];
                    if !before.is_empty() {
                        found_words.push(NoMatch(before));
                    }
                    found_words.push(Match(fragment));
                    string = &string[end_index..];
                    i = 0;
                    continue 'outer;
                }
            }

            i = string.ceil_char_boundary(i + 1);
        }

        if !string.is_empty() {
            found_words.push(NoMatch(string));
        }

        found_words
    }

    pub fn fuzzy_find_words(&self, mut string: &str) -> (Vec<&str>, usize) {
        let mut result = vec![];
        let mut offset = 0;
        let mut consumed = 0;
        // String has at least shortest_word_len graphemes
        while string
            .graphemes(true)
            .nth(self.shortest_word_len - 1)
            .is_some()
        {
            let word = self.fuzzy_match_word(string);
            if let Some((word, pattern)) = word {
                offset += word.len();
                consumed = offset;
                string = &string[word.len()..];
                result.push(pattern);
                debug_print!("remaining: {string}");
            } else {
                let next_index = string.ceil_char_boundary(1);
                offset += next_index;
                string = &string[next_index..];
            }
        }
        (result, consumed)
    }

    pub fn fuzzy_match_word<'dict, 'input>(
        &'dict self,
        input: &'input str,
    ) -> Option<(&'input str, &'dict str)> {
        debug_print!("Trying {input}");
        let mut lowest_distance = f64::MAX;
        let mut result = None;
        for (dict_word, len) in &self.words {
            let max_diff = max_lookahead(*len);
            for i in -2..=max_diff {
                let (mut last_index, grapheme) = input
                    .grapheme_indices(true)
                    .take((*len as i32 + i).max(0) as usize)
                    .last()
                    .expect("at least one grapheme");
                last_index += grapheme.len();

                let fragment = &input[..last_index];

                let distance = calculate_distance(fragment, dict_word);
                let threshold = get_threshold(*len);
                if distance <= threshold {
                    debug_print!(
                        "{fragment}..{dict_word} - Yep! - {distance:.3} <= {threshold:.3}"
                    );
                } else {
                    debug_print!(
                        "{fragment}..{dict_word} - Nope! - {distance:.3} > {threshold:.3}"
                    );
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SearchEntry<'a> {
    Match(&'a str),
    NoMatch(&'a str),
}

fn calculate_distance(string: &str, pattern: &str) -> f64 {
    1.0 - confusion::similarity(string, pattern)
}

fn max_lookahead(len: usize) -> i32 {
    (((len as f32).max(4.) - 4.) / 2.).ceil() as i32
}

const fn get_threshold(len: usize) -> f64 {
    len as f64 * 0.021
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
    use crate::dictionary::{Dictionary, SearchEntry};

    fn create_test_dict() -> Dictionary {
        Dictionary::from_vec(vec![
            "prizim".to_owned(),
            "fɛra".to_owned(),
            "kɛjfida".to_owned(),
        ])
    }

    #[track_caller]
    fn assert_on_dict(dict: &[&str], sequence: &str, remainder: &str, sequence_split: &[&str]) {
        let dict = Dictionary::from_vec(dict.iter().copied().map(ToOwned::to_owned).collect());
        assert_on_dict_(dict, sequence, remainder, sequence_split);
    }

    #[track_caller]
    fn assert_on_test_dict(sequence: &str, remainder: &str, sequence_split: &[&str]) {
        assert_on_dict_(create_test_dict(), sequence, remainder, sequence_split);
    }

    #[track_caller]
    fn assert_on_dict_(dict: Dictionary, sequence: &str, remainder: &str, sequence_split: &[&str]) {
        let (words, consumed) = dict.find_words_in_string(sequence);

        assert_eq!(&words, sequence_split);
        assert_eq!(&sequence[consumed..], remainder);
    }

    #[test]
    fn test_search_prizim_fera_kejfida() {
        const SEQUENCE: &str = "prizimfɛrakɛlfidapriziɲfɛrakɛjfidariziɲfɛrakɛjfidapɾizinfɛraːkaifiːdaːpriːzɨjimfɛraːkɛifiːdɛl";
        const REMAINDER: &str = "l";
        const SEQUENCE_SPLIT: &[&str] = &[
            "prizim", "fɛra", "kɛjfida", //
            "prizim", "fɛra", "kɛjfida", //
            "fɛra", "kɛjfida", //
            "prizim", "fɛra", "kɛjfida", //
            "prizim", "fɛra", "kɛjfida", //
        ];

        assert_on_test_dict(SEQUENCE, REMAINDER, SEQUENCE_SPLIT);
    }

    #[test]
    fn test_search_prizim_fera_kejfida_2() {
        const SEQUENCE: &str =
            "pɾizimfɛɾakɛlfidapɾilifɛrakajfidarizɲfɛrakajʃinakɾitunfɛɾaːkaiɸpɾiːzinferaːkɛlihinæl";
        const REMAINDER: &str = "ːkɛlihinæl";
        const SEQUENCE_SPLIT: &[&str] = &[
            "prizim", "fɛra", "kɛjfida", //
            "fɛra", "kɛjfida", //
            "fɛra", "kɛjfida", //
            "fɛra",    //
            "prizim", "fɛra", //
        ];

        assert_on_test_dict(SEQUENCE, REMAINDER, SEQUENCE_SPLIT);
    }

    #[test]
    fn test_search_split_prizim() {
        const SEQUENCE: &str = "pɾiːzim";
        const REMAINDER: &str = "";
        const SEQUENCE_SPLIT: &[&str] = &["prizim"];

        assert_on_test_dict(SEQUENCE, REMAINDER, SEQUENCE_SPLIT);
    }

    #[test]
    fn test_search_prilifera() {
        // In word "pɾilifɛra"
        // "pɾilif" is closer then "pɾili" to "prizim"
        // This test ensures that in such case "fɛra" has higher pririty (because it is an exact match)
        // and doesn't lose its f to "pɾilif"
        const SEQUENCE: &str = "pɾilifɛra";
        const REMAINDER: &str = "";
        const SEQUENCE_SPLIT: &[&str] = &["fɛra"];

        assert_on_test_dict(SEQUENCE, REMAINDER, SEQUENCE_SPLIT);
    }

    #[test]
    fn test_search_evi() {
        const DICT: &[&str] = &["evi"];
        const SEQUENCE: &str = "ɨraʑɲɛeitakpavinna";
        const REMAINDER: &str = SEQUENCE;
        const SEQUENCE_SPLIT: &[&str] = &[];

        assert_on_dict(DICT, SEQUENCE, REMAINDER, SEQUENCE_SPLIT);
    }

    #[test]
    fn test_search_fera() {
        const DICT: &[&str] = &["fɛra"];
        const SEQUENCE: &str = "ʂtut͡sɔʐkadaraztvatʂɨ";
        const REMAINDER: &str = SEQUENCE;
        const SEQUENCE_SPLIT: &[&str] = &[];

        assert_on_dict(DICT, SEQUENCE, REMAINDER, SEQUENCE_SPLIT);
    }

    #[test]
    fn test_search_liro() {
        const DICT: &[&str] = &["lirɔ"];
        const SEQUENCE: &str = "liːro";
        const REMAINDER: &str = "";
        const SEQUENCE_SPLIT: &[&str] = &["lirɔ"];

        assert_on_dict(DICT, SEQUENCE, REMAINDER, SEQUENCE_SPLIT);
    }

    #[test]
    fn test_search_no_match() {
        const SEQUENCE: &str = "Y]+g4Ty}F({7H!8nrn2(1ZH[Y)A0SSg4}0tXy!)013Vz}6kjZW(Fg{bpGY+D:Z1/X&5UmJ4L+X2=r8ji[a)h,i7[n7Ny9";
        const REMAINDER: &str = SEQUENCE;
        const SEQUENCE_SPLIT: &[&str] = &[];

        assert_on_test_dict(SEQUENCE, REMAINDER, SEQUENCE_SPLIT);
    }

    #[test]
    fn test_exact_search() {
        use SearchEntry::*;

        const SEQUENCE: &str = "prizimpɾilifɛrakɛjfidapɾilifɛrakɛlfida";
        const SEQUENCE_SPLIT: &[SearchEntry] = &[
            Match("prizim"),
            NoMatch("pɾili"),
            Match("fɛra"),
            Match("kɛjfida"),
            NoMatch("pɾili"),
            Match("fɛra"),
            NoMatch("kɛlfida"),
        ];

        let dict = create_test_dict();
        let words = dict.exact_find_words(SEQUENCE);

        assert_eq!(&words, SEQUENCE_SPLIT);
    }
}
