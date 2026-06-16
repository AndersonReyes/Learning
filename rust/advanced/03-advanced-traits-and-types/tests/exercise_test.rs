use advanced_03_advanced_traits_and_types::{
    greet_and_farewell, headlines, mat_mul, max_magnitude, top_n, Farewell, Greet, Magnitude,
    Matrix2x2, NewsArticle, Person, Summarize, WordCloud,
};
use std::collections::HashMap;

// --- Exercise 1: Magnitude ---------------------------------------------------

#[test]
fn magnitude_i64_positive() {
    assert_eq!(3_i64.magnitude(), 3_u64);
}

#[test]
fn magnitude_i64_negative() {
    assert_eq!((-7_i64).magnitude(), 7_u64);
}

#[test]
fn magnitude_i64_zero() {
    assert_eq!(0_i64.magnitude(), 0_u64);
}

#[test]
fn magnitude_f64_pair_3_4() {
    let v: (f64, f64) = (3.0, 4.0);
    assert!((v.magnitude() - 5.0_f64).abs() < 1e-9);
}

#[test]
fn magnitude_f64_pair_zero() {
    let v: (f64, f64) = (0.0, 0.0);
    assert!((v.magnitude() - 0.0_f64).abs() < 1e-9);
}

#[test]
fn magnitude_f64_pair_unit() {
    let v: (f64, f64) = (1.0, 0.0);
    assert!((v.magnitude() - 1.0_f64).abs() < 1e-9);
}

#[test]
fn max_magnitude_i64_typical() {
    assert_eq!(max_magnitude(&[3_i64, -7, 2]), Some(7_u64));
}

#[test]
fn max_magnitude_i64_empty() {
    assert_eq!(max_magnitude(&[] as &[i64]), None);
}

#[test]
fn max_magnitude_i64_single() {
    assert_eq!(max_magnitude(&[5_i64]), Some(5_u64));
}

#[test]
fn max_magnitude_i64_all_negative() {
    assert_eq!(max_magnitude(&[-1_i64, -5, -3]), Some(5_u64));
}

#[test]
fn max_magnitude_f64_pair() {
    let v = max_magnitude(&[(3.0_f64, 4.0_f64), (1.0, 0.0)]);
    assert!(v.is_some());
    assert!((v.unwrap() - 5.0_f64).abs() < 1e-9);
}

// --- Exercise 2: Matrix2x2 ---------------------------------------------------

fn mat(a: f64, b: f64, c: f64, d: f64) -> Matrix2x2 {
    Matrix2x2 { data: [[a, b], [c, d]] }
}

#[test]
fn matrix_add_typical() {
    let a = mat(1.0, 2.0, 3.0, 4.0);
    let b = mat(5.0, 6.0, 7.0, 8.0);
    assert_eq!((a + b).data, [[6.0, 8.0], [10.0, 12.0]]);
}

#[test]
fn matrix_add_zero() {
    let a = mat(1.0, 2.0, 3.0, 4.0);
    let z = mat(0.0, 0.0, 0.0, 0.0);
    assert_eq!((a + z).data, a.data);
}

#[test]
fn matrix_add_commutative() {
    let a = mat(1.0, 2.0, 3.0, 4.0);
    let b = mat(5.0, 6.0, 7.0, 8.0);
    assert_eq!((a + b).data, (b + a).data);
}

#[test]
fn matrix_scalar_mul_typical() {
    let a = mat(1.0, 2.0, 3.0, 4.0);
    assert_eq!((a * 2.0).data, [[2.0, 4.0], [6.0, 8.0]]);
}

#[test]
fn matrix_scalar_mul_zero() {
    let a = mat(1.0, 2.0, 3.0, 4.0);
    assert_eq!((a * 0.0).data, [[0.0, 0.0], [0.0, 0.0]]);
}

#[test]
fn matrix_scalar_mul_one() {
    let a = mat(1.0, 2.0, 3.0, 4.0);
    assert_eq!((a * 1.0).data, a.data);
}

#[test]
fn matrix_mul_typical() {
    let a = mat(1.0, 2.0, 3.0, 4.0);
    let b = mat(5.0, 6.0, 7.0, 8.0);
    assert_eq!(mat_mul(a, b).data, [[19.0, 22.0], [43.0, 50.0]]);
}

#[test]
fn matrix_mul_identity() {
    let a = mat(1.0, 2.0, 3.0, 4.0);
    let id = mat(1.0, 0.0, 0.0, 1.0);
    assert_eq!(mat_mul(a, id).data, a.data);
    assert_eq!(mat_mul(id, a).data, a.data);
}

#[test]
fn matrix_mul_by_zero_matrix() {
    let a = mat(1.0, 2.0, 3.0, 4.0);
    let z = mat(0.0, 0.0, 0.0, 0.0);
    assert_eq!(mat_mul(a, z).data, z.data);
}

// --- Exercise 3: WordCloud ---------------------------------------------------

fn make_wc(entries: &[(&str, usize)]) -> WordCloud {
    let mut m = HashMap::new();
    for (w, c) in entries {
        m.insert(w.to_string(), *c);
    }
    WordCloud(m)
}

#[test]
fn word_cloud_display_sorted() {
    let wc = make_wc(&[("rust", 3), ("go", 3), ("python", 1)]);
    let s = format!("{}", wc);
    assert_eq!(s, "go: 3\nrust: 3\npython: 1\n");
}

#[test]
fn word_cloud_display_single_entry() {
    let wc = make_wc(&[("hello", 5)]);
    assert_eq!(format!("{}", wc), "hello: 5\n");
}

#[test]
fn word_cloud_display_all_ties_alpha_order() {
    let wc = make_wc(&[("b", 2), ("a", 2), ("c", 2)]);
    assert_eq!(format!("{}", wc), "a: 2\nb: 2\nc: 2\n");
}

#[test]
fn top_n_returns_top_two() {
    let wc = make_wc(&[("rust", 3), ("go", 3), ("python", 1)]);
    let result = top_n(&wc, 2);
    assert_eq!(result, vec![("go", 3), ("rust", 3)]);
}

#[test]
fn top_n_zero_returns_empty() {
    let wc = make_wc(&[("rust", 3), ("go", 2)]);
    assert_eq!(top_n(&wc, 0), vec![]);
}

#[test]
fn top_n_exceeds_size_returns_all() {
    let wc = make_wc(&[("rust", 3), ("go", 2)]);
    let result = top_n(&wc, 100);
    assert_eq!(result, vec![("rust", 3), ("go", 2)]);
}

#[test]
fn top_n_single() {
    let wc = make_wc(&[("only", 7)]);
    assert_eq!(top_n(&wc, 1), vec![("only", 7)]);
}

// --- Exercise 4: Summarize / Supertrait -------------------------------------

fn article(title: &str, author: &str, words: u32) -> NewsArticle {
    NewsArticle { title: title.into(), author: author.into(), word_count: words }
}

#[test]
fn news_article_display() {
    let a = article("Rust 2024", "Alice", 500);
    assert_eq!(format!("{a}"), "Rust 2024 (500 words)");
}

#[test]
fn news_article_display_zero_words() {
    let a = article("Empty", "Bob", 0);
    assert_eq!(format!("{a}"), "Empty (0 words)");
}

#[test]
fn news_article_author() {
    let a = article("Title", "Carol", 100);
    assert_eq!(a.author(), "Carol");
}

#[test]
fn news_article_headline() {
    let a = article("Rust 2024", "Alice", 500);
    assert_eq!(a.headline(), "Rust 2024 (500 words) \u{2014} by Alice");
}

#[test]
fn headlines_single() {
    let items = vec![article("Rust 2024", "Alice", 500)];
    assert_eq!(headlines(&items), vec!["Rust 2024 (500 words) \u{2014} by Alice"]);
}

#[test]
fn headlines_multiple() {
    let items = vec![
        article("Rust 2024", "Alice", 500),
        article("Go 2025", "Bob", 300),
    ];
    assert_eq!(
        headlines(&items),
        vec![
            "Rust 2024 (500 words) \u{2014} by Alice",
            "Go 2025 (300 words) \u{2014} by Bob",
        ]
    );
}

#[test]
fn headlines_empty_slice() {
    let items: Vec<NewsArticle> = vec![];
    assert_eq!(headlines(&items), Vec::<String>::new());
}

// --- Exercise 5: Fully qualified syntax -------------------------------------

#[test]
fn greet_message() {
    let p = Person { name: "Alice".into() };
    assert_eq!(<Person as Greet>::message(&p), "Hello, Alice!");
}

#[test]
fn farewell_message() {
    let p = Person { name: "Alice".into() };
    assert_eq!(<Person as Farewell>::message(&p), "Goodbye, Alice!");
}

#[test]
fn greet_and_farewell_alice() {
    let p = Person { name: "Alice".into() };
    assert_eq!(
        greet_and_farewell(&p),
        ("Hello, Alice!".into(), "Goodbye, Alice!".into())
    );
}

#[test]
fn greet_and_farewell_bob() {
    let p = Person { name: "Bob".into() };
    assert_eq!(
        greet_and_farewell(&p),
        ("Hello, Bob!".into(), "Goodbye, Bob!".into())
    );
}

#[test]
fn greet_and_farewell_empty_name() {
    let p = Person { name: "".into() };
    assert_eq!(
        greet_and_farewell(&p),
        ("Hello, !".into(), "Goodbye, !".into())
    );
}
