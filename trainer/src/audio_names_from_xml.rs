use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    str::from_utf8,
};

use quick_xml::{Reader, escape::unescape, events::Event};
use regex::Regex;

pub fn audio_names_from_xml(
    input_file: &str,
    dict: &HashMap<String, String>,
    output_file: &str,
    lang: &str,
) -> io::Result<()> {
    let mut result: HashMap<String, String> = HashMap::new();
    let mut found = 0;

    let file = BufReader::new(File::open(input_file)?);
    let mut reader = Reader::from_reader(file);
    reader.config_mut().trim_text(false);

    let mut buf = Vec::new();
    let mut current_title: Option<String> = None;
    let mut in_title = false;
    let mut in_text = false;
    let mut text_buf = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => match e.local_name().as_ref() {
                b"title" => {
                    in_title = true;
                    current_title = Some(String::new());
                }
                b"text" => {
                    in_text = true;
                    text_buf.clear();
                }
                _ => {}
            },
            Ok(Event::Text(ref e)) => {
                if in_title {
                    if let Some(ref mut t) = current_title {
                        t.push_str(&unescape(from_utf8(e.as_ref()).unwrap()).unwrap_or_default());
                    }
                } else if in_text {
                    text_buf.push_str(from_utf8(e.as_ref()).unwrap());
                }
            }
            Ok(Event::End(ref e)) => {
                match e.local_name().as_ref() {
                    b"title" => in_title = false,
                    b"text" => {
                        in_text = false;
                        if let Some(ref title) = current_title
                            && dict.contains_key(&title.to_lowercase())
                                && !title.contains(':') // Skip helper pages (Wiktionary:, Template:, etc.)
                                && let Some(audio) =
                                    Wikitext(&text_buf).extract_audio_url(lang, title)
                        {
                            result.insert(title.clone(), audio);
                            found += 1;
                        }
                    }
                    b"page" => {
                        current_title = None;
                        text_buf.clear();
                    }
                    _ => {}
                }
            }
            Ok(Event::GeneralRef(ref e)) => {
                let text = from_utf8(e.as_ref()).unwrap();
                match text {
                    "lt" => text_buf.push('<'),
                    "gt" => text_buf.push('>'),
                    "quot" => text_buf.push('"'),
                    "amp" => text_buf.push('&'),
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("XML parse error: {e}");
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    println!("Done, found audio for {found}/{} dict entries", dict.len());

    let mut writer = csv::Writer::from_path(output_file)?;
    writer.write_record(["word", "ipa", "file"])?;

    let mut words: Vec<&String> = result.keys().collect();
    words.sort_unstable();
    for word in words {
        writer.write_record([word, &dict[word], &result[word]])?;
    }
    writer.flush()?;

    Ok(())
}

pub struct Wikitext<'a>(pub &'a str);

impl<'a> Wikitext<'a> {
    pub fn extract_audio_url(&self, lang_name: &str, page_title: &str) -> Option<String> {
        let section_header = format!("=={lang_name}==");
        let section_start = self.0.find(&section_header)?;

        let after_header = &self.0[section_start + section_header.len()..];
        let section_end = Wikitext(after_header).find_next_top_level_section();
        let section = Wikitext(&after_header[..section_end]);

        let macro_section = section.find_pl_pr_section()?.0;

        if macro_section.is_empty() {
            return None;
        }

        parse_pl_pr_macro(page_title, macro_section)
    }

    fn find_pl_pr_section(&self) -> Option<Wikitext<'_>> {
        const MACRO_START: &str = "{{pl-pr";

        let start = self.0.find(MACRO_START)? + MACRO_START.len();

        let mut depth = 0;
        let bracket_regex = Regex::new(r"\{\{|\}\}").unwrap();

        for next_match in bracket_regex.find_iter(&self.0[start..]) {
            let is_opening = next_match.as_str().as_bytes()[0] == b'{';
            match (depth, is_opening) {
                (0, false) => {
                    return Some(Wikitext(&self.0[start..(start + next_match.start())]));
                }
                (_, true) => {
                    depth += 1;
                }
                (_, false) => {
                    depth -= 1;
                }
            }
        }

        None
    }

    fn find_next_top_level_section(&self) -> usize {
        let mut i = 0;
        let bytes = self.0.as_bytes();
        while i < bytes.len() {
            if bytes[i] == b'\n'
                && i + 3 < bytes.len()
                && bytes[i + 1] == b'='
                && bytes[i + 2] == b'='
                && bytes[i + 3] != b'='
            {
                return i;
            }
            i += 1;
        }
        self.0.len()
    }
}

fn parse_pl_pr_macro(page_title: &str, macro_section: &str) -> Option<String> {
    let audio_section = macro_section
        .split("|")
        .find(|frag| frag.starts_with("a=") || frag.starts_with("audio="))?;
    let stripped = audio_section
        .strip_prefix("a=")
        .or_else(|| audio_section.strip_prefix("audio="))?;
    let first = stripped.split(";").next()?;
    let mapped = first.replace('#', page_title);

    let modifier_re = Regex::new(r"<[^>]+>").unwrap();
    let clean = modifier_re.replace_all(&mapped, "").to_string();
    Some(clean)
}

#[test]
fn test_pl_pr_parsing() {
    let input = "pl-pr|a=Pl-#.ogg<text:#>;LL-Q809 (pol)-Olaf-#.wav<text:#>;LL-Q809 (pol)-Olaf-# się.wav<text:~>|kuj=#|mp=#";
    let parsed = parse_pl_pr_macro("budzić", input).unwrap();
    assert_eq!(parsed, "Pl-budzić.ogg");
}
