use advanced_07_advanced_lifetimes_variance_and_phantomdata::{
    apply_all_refs, longest_with_announcement, split_fields, Record, StrSplit, Token,
};

// --- Exercise 1: longest_with_announcement -----------------------------------

#[test]
fn longest_with_ann_x_wins() {
    let x = "long string";
    let y = "xy";
    let result = longest_with_announcement(x, y, "comparing");
    assert_eq!(result, "long string");
}

#[test]
fn longest_with_ann_y_wins() {
    let x = "hi";
    let y = "hello world";
    let result = longest_with_announcement(x, y, "testing");
    assert_eq!(result, "hello world");
}

#[test]
fn longest_with_ann_equal_len() {
    let x = "abc";
    let y = "xyz";
    let result = longest_with_announcement(x, y, "tie");
    assert_eq!(result.len(), 3);
}

#[test]
fn longest_with_ann_empty() {
    let result = longest_with_announcement("", "", "empty");
    assert_eq!(result, "");
}

// --- Exercise 2: StrSplit ---------------------------------------------------

#[test]
fn str_split_basic() {
    let s = StrSplit::new("a,b,c", ",");
    assert_eq!(s.collect::<Vec<_>>(), vec!["a", "b", "c"]);
}

#[test]
fn str_split_no_delimiter() {
    let s = StrSplit::new("no-delimiter", "X");
    assert_eq!(s.collect::<Vec<_>>(), vec!["no-delimiter"]);
}

#[test]
fn str_split_leading_delimiter() {
    let s = StrSplit::new(",leading", ",");
    assert_eq!(s.collect::<Vec<_>>(), vec!["", "leading"]);
}

#[test]
fn str_split_trailing_delimiter() {
    let s = StrSplit::new("trailing,", ",");
    assert_eq!(s.collect::<Vec<_>>(), vec!["trailing", ""]);
}

#[test]
fn str_split_multi_char_delimiter() {
    let s = StrSplit::new("a::b::c", "::");
    assert_eq!(s.collect::<Vec<_>>(), vec!["a", "b", "c"]);
}

#[test]
fn str_split_empty_haystack() {
    let s = StrSplit::new("", ",");
    assert_eq!(s.collect::<Vec<_>>(), vec![""]);
}

#[test]
fn str_split_only_delimiters() {
    let s = StrSplit::new(",,", ",");
    assert_eq!(s.collect::<Vec<_>>(), vec!["", "", ""]);
}

// --- Exercise 3: Token (PhantomData branding) --------------------------------

#[derive(Copy, Clone)]
struct SessionId;
#[derive(Copy, Clone)]
struct RequestId;

#[test]
fn token_new_and_value() {
    let t: Token<SessionId> = Token::new(42);
    assert_eq!(t.value(), 42);
}

#[test]
fn token_different_brands_same_value() {
    let s: Token<SessionId> = Token::new(99);
    let r: Token<RequestId> = Token::new(99);
    assert_eq!(s.value(), r.value());
}

#[test]
fn token_zero() {
    let t: Token<SessionId> = Token::new(0);
    assert_eq!(t.value(), 0);
}

#[test]
fn token_max() {
    let t: Token<RequestId> = Token::new(u64::MAX);
    assert_eq!(t.value(), u64::MAX);
}

#[test]
fn token_copy() {
    let t: Token<SessionId> = Token::new(7);
    let u = t; // Token<SessionId>: Copy because SessionId: Copy
    assert_eq!(t.value(), u.value());
}

// --- Exercise 4: split_fields -----------------------------------------------

#[test]
fn split_fields_modify_both() {
    let mut r = Record { name: "Alice".into(), description: "Engineer".into() };
    let (name, desc) = split_fields(&mut r);
    name.push_str(" B.");
    desc.push_str("ing");
    assert_eq!(r.name, "Alice B.");
    assert_eq!(r.description, "Engineering");
}

#[test]
fn split_fields_returns_correct_refs() {
    let mut r = Record { name: "Bob".into(), description: "Chef".into() };
    let (name, desc) = split_fields(&mut r);
    assert_eq!(name.as_str(), "Bob");
    assert_eq!(desc.as_str(), "Chef");
}

#[test]
fn split_fields_empty() {
    let mut r = Record { name: String::new(), description: String::new() };
    let (name, desc) = split_fields(&mut r);
    name.push('X');
    desc.push('Y');
    assert_eq!(r.name, "X");
    assert_eq!(r.description, "Y");
}

// --- Exercise 5: apply_all_refs (HRTB) --------------------------------------

#[test]
fn apply_all_refs_identity_i32() {
    let nums = vec![1_i32, 2, 3];
    let result = apply_all_refs(&nums, |n| n);
    // result: Vec<&i32>
    assert_eq!(result, vec![&1_i32, &2, &3]);
}

#[test]
fn apply_all_refs_identity_str_slice() {
    let words = vec!["hello".to_string(), "world".to_string()];
    let result = apply_all_refs(&words, |s| s);
    // result: Vec<&String>; deref-compare
    assert_eq!(result[0].as_str(), "hello");
    assert_eq!(result[1].as_str(), "world");
}

#[test]
fn apply_all_refs_empty() {
    let empty: Vec<i32> = vec![];
    let result = apply_all_refs(&empty, |n| n);
    assert_eq!(result, Vec::<&i32>::new());
}

#[test]
fn apply_all_refs_single() {
    let v = vec![42_i32];
    let result = apply_all_refs(&v, |n| n);
    assert_eq!(result, vec![&42_i32]);
}
