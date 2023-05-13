// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

fn main() {
    let words: [&str; 2] = [
        "first",
        "apple"
    ];

    for word in words.iter() {
        println!("{}", pig_latin(word))
    }
}

fn pig_latin(word: &str) -> String {
    // if (word.next())
    match word.chars().nth(0).unwrap() {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-fay", word[1..].to_string())
    }

}