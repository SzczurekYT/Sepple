use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn convert_to_ipa_dictionary(input_file: &str, output_file: &str) -> io::Result<()> {
    let mut result: HashMap<String, String> = HashMap::new();
    let mut skipped = 0;
    for line in BufReader::new(File::open(input_file).unwrap()).lines() {
        let line = line?;
        let json: serde_json::Value = serde_json::from_str(&line).unwrap();
        let json = json.as_object().unwrap();
        let word = json["word"].as_str().unwrap();

        let ipa: Option<String> = json.get("sounds").and_then(|list| {
            let mut i = 0;
            list.as_array()
                .expect("an array")
                .iter()
                .filter_map(|object| {
                    object
                        .as_object()
                        .expect("an object")
                        .get("ipa")
                        .map(|ipa| ipa.as_str().unwrap().to_owned())
                })
                .inspect(|ipa| {
                    i += 1;
                    if i > 1 {
                        println!("{word} has more then 1 ipa, {ipa}")
                    }
                })
                .next()
        });

        let Some(ipa) = ipa else {
            skipped += 1;
            continue;
        };
        result.insert(word.to_owned(), ipa.to_owned());
    }

    println!(
        "Done, skipped {skipped} entries out of {} due to missing IPA",
        skipped + result.len()
    );

    serde_json::to_writer(File::create(output_file)?, &result)?;

    Ok(())
}
