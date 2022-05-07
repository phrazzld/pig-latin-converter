use std::io;

fn main() {
    println!("Enter a word to convert it to Pig Latin:");

    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line.");

    let word: String = word.trim().parse().expect("Failed to parse word.");

    println!("You entered: {}", word);

    let mut found_vowel = false;
    let mut core_word = String::new();
    let mut append_chars = String::new();

    for c in word.chars() {
        if is_vowel(c) {
            found_vowel = true;
        }

        if found_vowel {
            core_word.push(c);
        } else {
            append_chars.push(c);
        }
    }

    let starts_with_vowel = word.eq(&core_word);

    // If the word starts with a vowel, append "hay"
    if starts_with_vowel {
        core_word.push_str("hay");
    } else {
        // Otherwise, slice up to the first vowel
        // And append the slice plus "ay" to the end of the word
        core_word.push_str(&append_chars);
        core_word.push_str("ay");
    }

    println!("{} in Pig Latin is {}", word, core_word);
}

fn is_vowel(c: char) -> bool {
    "AEIOUaeiou".find(c) != None
}
