use fundamentals_09_common_collections::{
    dedup_preserve_order, group_anagrams, run_length_encode, top_k_frequent, word_frequency,
};

#[test]
fn word_frequency_case_insensitive() {
    let counts = word_frequency("The the THE");
    assert_eq!(counts.len(), 1);
    assert_eq!(counts.get("the"), Some(&3));
}

#[test]
fn word_frequency_strips_punctuation() {
    let counts = word_frequency("Hello, hello! HELLO?");
    assert_eq!(counts.len(), 1);
    assert_eq!(counts.get("hello"), Some(&3));
}

#[test]
fn word_frequency_multiple_words() {
    let counts = word_frequency("a b c a b a");
    assert_eq!(counts.get("a"), Some(&3));
    assert_eq!(counts.get("b"), Some(&2));
    assert_eq!(counts.get("c"), Some(&1));
    assert_eq!(counts.len(), 3);
}

#[test]
fn word_frequency_empty_string() {
    assert_eq!(word_frequency("").len(), 0);
}

#[test]
fn word_frequency_sentence() {
    let counts = word_frequency("The quick brown fox jumps over the lazy dog. The dog barks.");
    assert_eq!(counts.get("the"), Some(&3));
    assert_eq!(counts.get("dog"), Some(&2));
    assert_eq!(counts.get("quick"), Some(&1));
    assert_eq!(counts.get("barks"), Some(&1));
}

fn strings(words: &[&str]) -> Vec<String> {
    words.iter().map(|s| s.to_string()).collect()
}

#[test]
fn group_anagrams_basic() {
    let words = strings(&["eat", "tea", "tan", "ate", "nat", "bat"]);
    assert_eq!(
        group_anagrams(&words),
        vec![
            strings(&["eat", "tea", "ate"]),
            strings(&["tan", "nat"]),
            strings(&["bat"]),
        ]
    );
}

#[test]
fn group_anagrams_empty_input() {
    assert_eq!(group_anagrams(&[] as &[String]), Vec::<Vec<String>>::new());
}

#[test]
fn group_anagrams_single_word() {
    let words = strings(&["abc"]);
    assert_eq!(group_anagrams(&words), vec![strings(&["abc"])]);
}

#[test]
fn group_anagrams_all_same_anagram_class() {
    let words = strings(&["ab", "ba", "ab"]);
    assert_eq!(group_anagrams(&words), vec![strings(&["ab", "ba", "ab"])]);
}

#[test]
fn top_k_frequent_basic() {
    assert_eq!(top_k_frequent(&[1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
}

#[test]
fn top_k_frequent_ties_broken_by_value_ascending() {
    assert_eq!(top_k_frequent(&[1, 2, 3, 4], 2), vec![1, 2]);
}

#[test]
fn top_k_frequent_mixed_ties() {
    // 2 and 4 both occur 3 times; 1 occurs twice.
    assert_eq!(top_k_frequent(&[4, 4, 4, 1, 1, 2, 2, 2], 2), vec![2, 4]);
}

#[test]
fn top_k_frequent_k_zero_is_empty() {
    assert_eq!(top_k_frequent(&[1, 2, 3], 0), Vec::<i32>::new());
}

#[test]
fn top_k_frequent_single_value() {
    assert_eq!(top_k_frequent(&[1], 1), vec![1]);
}

#[test]
fn dedup_preserve_order_basic() {
    assert_eq!(dedup_preserve_order(&[1, 2, 1, 3, 2, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn dedup_preserve_order_all_same() {
    assert_eq!(dedup_preserve_order(&[5, 5, 5]), vec![5]);
}

#[test]
fn dedup_preserve_order_no_duplicates() {
    assert_eq!(dedup_preserve_order(&[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn dedup_preserve_order_empty() {
    assert_eq!(dedup_preserve_order(&[]), Vec::<i32>::new());
}

#[test]
fn run_length_encode_basic() {
    assert_eq!(run_length_encode("aaabbc"), "3a2b1c");
}

#[test]
fn run_length_encode_single_char() {
    assert_eq!(run_length_encode("a"), "1a");
}

#[test]
fn run_length_encode_empty_string() {
    assert_eq!(run_length_encode(""), "");
}

#[test]
fn run_length_encode_no_repeats() {
    assert_eq!(run_length_encode("abcd"), "1a1b1c1d");
}

#[test]
fn run_length_encode_multibyte_utf8() {
    assert_eq!(run_length_encode("aabb🦀🦀🦀c"), "2a2b3🦀1c");
}
