use intermediate_01_generics_traits_and_lifetimes::{sum_all, Bst, Money, Tokenizer};

#[test]
fn bst_insert_and_in_order_with_duplicates() {
    let mut tree = Bst::new();
    for v in [5, 3, 8, 1, 4, 7, 9, 3, 5] {
        tree.insert(v);
    }
    assert_eq!(tree.in_order(), vec![&1, &3, &4, &5, &7, &8, &9]);
}

#[test]
fn bst_contains() {
    let mut tree = Bst::new();
    for v in [5, 3, 8, 1, 4, 7, 9] {
        tree.insert(v);
    }
    assert!(tree.contains(&4));
    assert!(tree.contains(&9));
    assert!(!tree.contains(&6));
    assert!(!tree.contains(&0));
}

#[test]
fn bst_empty_tree() {
    let tree: Bst<i32> = Bst::new();
    assert_eq!(tree.in_order(), Vec::<&i32>::new());
    assert!(!tree.contains(&1));
}

#[test]
fn bst_single_element() {
    let mut tree = Bst::new();
    tree.insert(42);
    assert_eq!(tree.in_order(), vec![&42]);
    assert!(tree.contains(&42));
    assert!(!tree.contains(&41));
}

#[test]
fn bst_works_with_str() {
    let mut tree = Bst::new();
    for s in ["banana", "apple", "cherry", "apple"] {
        tree.insert(s);
    }
    assert_eq!(tree.in_order(), vec![&"apple", &"banana", &"cherry"]);
    assert!(tree.contains(&"cherry"));
    assert!(!tree.contains(&"date"));
}

#[test]
fn bst_default_is_empty() {
    let tree: Bst<i32> = Bst::default();
    assert_eq!(tree.in_order(), Vec::<&i32>::new());
}

#[test]
fn bst_ascending_insert_order() {
    let mut tree = Bst::new();
    for v in 1..=5 {
        tree.insert(v);
    }
    assert_eq!(tree.in_order(), vec![&1, &2, &3, &4, &5]);
}

#[test]
fn bst_descending_insert_order() {
    let mut tree = Bst::new();
    for v in (1..=5).rev() {
        tree.insert(v);
    }
    assert_eq!(tree.in_order(), vec![&1, &2, &3, &4, &5]);
}

#[test]
fn sum_all_integers() {
    assert_eq!(sum_all(&[1, 2, 3, 4]), 10);
}

#[test]
fn sum_all_empty_slice() {
    assert_eq!(sum_all::<i32>(&[]), 0);
}

#[test]
fn sum_all_floats() {
    assert_eq!(sum_all(&[1.5_f64, 2.5]), 4.0);
}

#[test]
fn sum_all_negative_numbers() {
    assert_eq!(sum_all(&[-5, 10, -3]), 2);
}

#[test]
fn sum_all_single_element() {
    assert_eq!(sum_all(&[42]), 42);
}

#[test]
fn sum_all_custom_type_money() {
    let amounts = [
        Money { cents: 100 },
        Money { cents: 250 },
        Money { cents: 50 },
    ];
    assert_eq!(sum_all(&amounts), Money { cents: 400 });
}

#[test]
fn sum_all_empty_money_slice_is_default() {
    assert_eq!(sum_all::<Money>(&[]), Money::default());
}

#[test]
fn tokenizer_basic() {
    let mut t = Tokenizer::new("Hello, world! 123-abc");
    assert_eq!(t.next_token(), Some("Hello"));
    assert_eq!(t.next_token(), Some("world"));
    assert_eq!(t.next_token(), Some("123"));
    assert_eq!(t.next_token(), Some("abc"));
    assert_eq!(t.next_token(), None);
    assert_eq!(t.next_token(), None);
}

#[test]
fn tokenizer_empty_input() {
    let mut t = Tokenizer::new("");
    assert_eq!(t.next_token(), None);
}

#[test]
fn tokenizer_only_separators() {
    let mut t = Tokenizer::new("   ...   ");
    assert_eq!(t.next_token(), None);
}

#[test]
fn tokenizer_leading_and_trailing_whitespace() {
    let mut t = Tokenizer::new("  abc  ");
    assert_eq!(t.next_token(), Some("abc"));
    assert_eq!(t.next_token(), None);
}

#[test]
fn tokenizer_alphanumeric_runs_combine_letters_and_digits() {
    let mut t = Tokenizer::new("abc123 def456");
    assert_eq!(t.next_token(), Some("abc123"));
    assert_eq!(t.next_token(), Some("def456"));
    assert_eq!(t.next_token(), None);
}

#[test]
fn tokenizer_non_ascii_acts_as_separator() {
    let mut t = Tokenizer::new("café 42");
    assert_eq!(t.next_token(), Some("caf"));
    assert_eq!(t.next_token(), Some("42"));
    assert_eq!(t.next_token(), None);
}

#[test]
fn tokenizer_underscore_is_a_separator() {
    let mut t = Tokenizer::new("_underscore_");
    assert_eq!(t.next_token(), Some("underscore"));
    assert_eq!(t.next_token(), None);
}

#[test]
fn tokenizer_returned_slices_borrow_from_input() {
    let input = String::from("foo bar");
    let mut t = Tokenizer::new(&input);
    let first = t.next_token().unwrap();
    let second = t.next_token().unwrap();
    assert_eq!(first, "foo");
    assert_eq!(second, "bar");
    // both slices borrow directly from `input`'s storage
    assert_eq!(first.as_ptr(), input.as_ptr());
}
