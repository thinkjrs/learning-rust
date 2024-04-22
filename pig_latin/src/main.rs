// Convert strings to pig latin. The first consonant of each word is moved to
// the end of the word and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead
// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
use unicode_segmentation::UnicodeSegmentation;

const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

fn convert_word_to_pig_latin(string_to_convert: &String) -> String {
    let mut result = String::new();
    // construct first word
    for (i, c) in string_to_convert.chars().enumerate() {
        if i == 0 {
            if VOWELS.contains(&c) {
                // append -hay and return
                result = String::from(format!("{}-hay", string_to_convert))
            } else {
                let mut iter = string_to_convert.grapheme_indices(true);
                iter.next();
                result = String::from(format!("{}-{}ay", iter.as_str(), c));
            }
        }
    }
    // construct second word including hyphen
    result
}

fn convert_to_pig_latin(string_to_convert: &String) -> String {
    let words = string_to_convert.unicode_words().collect::<Vec<&str>>();
    let mut result: String = String::new();
    for (i, word) in words.iter().enumerate() {
        match i {
            0 => {
                result = convert_word_to_pig_latin(&String::from(*word));
            }
            _ => {
                result += &format!(" {}", &convert_word_to_pig_latin(&String::from(*word)));
            }
        }
    }
    result
}
fn main() {
    let mut test_string_1 = String::from("first");
    let test_result_1 = String::from("irst-fay");
    assert_eq!(convert_to_pig_latin(&mut test_string_1), test_result_1);

    let mut test_string_2 = String::from("apple");
    let test_result_2 = String::from("apple-hay");
    assert_eq!(convert_to_pig_latin(&mut test_string_2), test_result_2);

    let mut test_string_3 = String::from("Apples are first");
    let test_result_3 = String::from("Apples-hay are-hay irst-fay");
    assert_eq!(convert_to_pig_latin(&mut test_string_3), test_result_3);
}
