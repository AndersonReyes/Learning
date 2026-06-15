use intermediate_05_custom_iterators_and_adapters::{chunks, fibonacci, pairwise, run_length, Grid};

// --- Fibonacci -----------------------------------------------------------------

#[test]
fn fibonacci_first_ten() {
    let first10: Vec<u64> = fibonacci().take(10).collect();
    assert_eq!(first10, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
}

#[test]
fn fibonacci_single_value() {
    let first: Vec<u64> = fibonacci().take(1).collect();
    assert_eq!(first, vec![0]);
}

#[test]
fn fibonacci_nth() {
    // index (0-based): 0,1,1,2,3,5,8,13,21,34,55 -- nth(10) is the 11th value
    assert_eq!(fibonacci().nth(10), Some(55));
}

#[test]
fn fibonacci_sum_of_first_fifteen() {
    let sum: u64 = fibonacci().take(15).sum();
    assert_eq!(sum, 986);
}

// --- Pairwise --------------------------------------------------------------------

#[test]
fn pairwise_basic() {
    let pairs: Vec<(i32, i32)> = pairwise(vec![1, 2, 3, 4].into_iter()).collect();
    assert_eq!(pairs, vec![(1, 2), (2, 3), (3, 4)]);
}

#[test]
fn pairwise_single_element_returns_empty() {
    let pairs: Vec<(i32, i32)> = pairwise(vec![1].into_iter()).collect();
    assert_eq!(pairs, Vec::<(i32, i32)>::new());
}

#[test]
fn pairwise_empty_returns_empty() {
    let pairs: Vec<(i32, i32)> = pairwise(Vec::<i32>::new().into_iter()).collect();
    assert_eq!(pairs, Vec::<(i32, i32)>::new());
}

#[test]
fn pairwise_two_elements() {
    let pairs: Vec<(i32, i32)> = pairwise(vec![5, 10].into_iter()).collect();
    assert_eq!(pairs, vec![(5, 10)]);
}

#[test]
fn pairwise_with_strings() {
    let words = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let pairs: Vec<(String, String)> = pairwise(words.into_iter()).collect();
    assert_eq!(
        pairs,
        vec![
            ("a".to_string(), "b".to_string()),
            ("b".to_string(), "c".to_string())
        ]
    );
}

// --- RunLength -------------------------------------------------------------------

#[test]
fn run_length_basic() {
    let runs: Vec<(char, usize)> = run_length("aaabccccd".chars()).collect();
    assert_eq!(runs, vec![('a', 3), ('b', 1), ('c', 4), ('d', 1)]);
}

#[test]
fn run_length_empty() {
    let runs: Vec<(char, usize)> = run_length("".chars()).collect();
    assert_eq!(runs, Vec::<(char, usize)>::new());
}

#[test]
fn run_length_all_same() {
    let runs: Vec<(char, usize)> = run_length("aaaa".chars()).collect();
    assert_eq!(runs, vec![('a', 4)]);
}

#[test]
fn run_length_no_repeats() {
    let runs: Vec<(char, usize)> = run_length("abcd".chars()).collect();
    assert_eq!(runs, vec![('a', 1), ('b', 1), ('c', 1), ('d', 1)]);
}

#[test]
fn run_length_with_numbers() {
    let runs: Vec<(i32, usize)> = run_length(vec![1, 1, 2, 2, 2, 3].into_iter()).collect();
    assert_eq!(runs, vec![(1, 2), (2, 3), (3, 1)]);
}

// --- ChunksIterator ----------------------------------------------------------------

#[test]
fn chunks_basic_uneven() {
    let result: Vec<Vec<i32>> = chunks(vec![1, 2, 3, 4, 5].into_iter(), 2).collect();
    assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5]]);
}

#[test]
fn chunks_exact_multiple() {
    let result: Vec<Vec<i32>> = chunks(vec![1, 2, 3, 4].into_iter(), 2).collect();
    assert_eq!(result, vec![vec![1, 2], vec![3, 4]]);
}

#[test]
fn chunks_size_larger_than_input() {
    let result: Vec<Vec<i32>> = chunks(vec![1, 2, 3].into_iter(), 5).collect();
    assert_eq!(result, vec![vec![1, 2, 3]]);
}

#[test]
fn chunks_empty_input() {
    let result: Vec<Vec<i32>> = chunks(Vec::<i32>::new().into_iter(), 3).collect();
    assert_eq!(result, Vec::<Vec<i32>>::new());
}

#[test]
fn chunks_size_one() {
    let result: Vec<Vec<i32>> = chunks(vec![1, 2, 3].into_iter(), 1).collect();
    assert_eq!(result, vec![vec![1], vec![2], vec![3]]);
}

#[test]
fn chunks_size_zero_yields_nothing() {
    let result: Vec<Vec<i32>> = chunks(vec![1, 2, 3].into_iter(), 0).collect();
    assert_eq!(result, Vec::<Vec<i32>>::new());
}

// --- Grid + IntoIterator ------------------------------------------------------------

#[test]
fn grid_basic_row_major() {
    let grid = Grid::new(vec![vec![1, 2], vec![3, 4, 5]]);
    let flat: Vec<i32> = grid.into_iter().collect();
    assert_eq!(flat, vec![1, 2, 3, 4, 5]);
}

#[test]
fn grid_empty_rows_skipped() {
    let grid = Grid::new(vec![vec![], vec![1], vec![]]);
    let flat: Vec<i32> = grid.into_iter().collect();
    assert_eq!(flat, vec![1]);
}

#[test]
fn grid_completely_empty() {
    let grid = Grid::new(vec![]);
    let flat: Vec<i32> = grid.into_iter().collect();
    assert_eq!(flat, Vec::<i32>::new());
}

#[test]
fn grid_single_row() {
    let grid = Grid::new(vec![vec![10, 20, 30]]);
    let flat: Vec<i32> = grid.into_iter().collect();
    assert_eq!(flat, vec![10, 20, 30]);
}

#[test]
fn grid_for_loop() {
    let grid = Grid::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    let mut sum = 0;
    for x in grid {
        sum += x;
    }
    assert_eq!(sum, 21);
}
