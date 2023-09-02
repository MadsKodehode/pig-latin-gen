use regex::Regex;
fn main() {
    let re = Regex::new(r"[\s,]+").expect("Invalid regex pattern");

    let mut buf = String::new();

    println!("Enter some words to translate, separated by commas or spaces");

    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");

    buf = buf.trim().to_string();

    let mut words: Vec<String> = re.split(&buf).map(str::to_string).collect();

    let translated = pig(&mut words);

    println!("{translated}");
}

fn is_vowel(c: &char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    vowels.contains(c)
}

fn pig(words: &mut [String]) -> String {
    let mut pigged = String::new();
    for w in words {
        pigged.push_str(&format!("\n({w}): "));
        let first_char = w.remove(0);

        if is_vowel(&first_char) {
            pigged.push(first_char);
            pigged.push_str(&format!("{w}-hay "));
        } else {
            pigged.push_str(&format!("{w}-{first_char}ay "));
        }
    }

    pigged
}
