import Levenshtein
import json

with open("dictionary.json", "r") as f:
    word_list_text = f.read()

words: list[str] = json.loads(word_list_text)

sequence = input("Sequence: ")
sequence = sequence.replace(":", "")


def matches(string: str, pattern: str) -> bool:
    max_difference = 1 if len(string) < 6 else 2 if len(string) < 9 else 3
    distance = Levenshtein.distance(string, pattern, score_cutoff=max_difference)
    return distance <= max_difference


while len(sequence) != 0:
    found = False
    for word in words:
        word_len = len(word)
        sequence_word = sequence[:word_len]
        if matches(sequence_word, word):
            print(f"Found: {word}")
            found = True
            sequence = sequence[word_len:]
            # print(f"Remaining 1: {sequence}")
            break
    if found:
        continue
    sequence = sequence[1:]
    # print(f"Remaining 2: {sequence}")
