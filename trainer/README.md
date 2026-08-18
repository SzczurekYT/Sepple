This is an utility tool for downloading and extracting training data from Wiktionary dumps.

## Setup - files needed
```shell
mkdir training && cd training/
# Raw pages dump for extracting audio urls (1.51G download, 12G uncompressed)
wget https://dumps.wikimedia.org/enwiktionary/latest/enwiktionary-latest-pages-articles.xml.bz2
bzip2 -d enwiktionary-latest-pages-articles.xml.bz2
# Wiktionary words with IPA (618M)
# Note: This download is deprecated, but it is convenient
# so let's use it while it exists ツ
wget https://kaikki.org/dictionary/Polish/kaikki.org-dictionary-Polish.jsonl
# Tar with audio files (20.4GB)
wget https://kaikki.org/dictionary/wiktionary-audios.tar
```

## Preparing the data
```shell
cd training
# Get a word to IPA dictionary
cargo run -r -p trainer ipa-from-jsonl kaikki.org-dictionary-Polish.jsonl ipa-dictionary-pl.json
# Get word, ipa and file name of speech audio (writes a csv: word,ipa,file)
cargo run -r -p trainer audio-names-from-xml ipa-dictionary-pl.json enwiktionary-latest-pages-articles.xml filenames.csv Polish
# Extract audio files (writes filenames_found.csv with word,ipa,file of extracted files only)
cargo run -r -p trainer extract-audio-tar wiktionary-audios.tar filenames.csv audio_data filenames_found.csv
```
