//! Run with: `cargo run --example examples -p intermediate-07-cargo-workspaces-and-profiles`

// --- ch.13.4: loop vs. iterator-chain versions of the same function ----------------

/// Loop version: lines of `contents` containing `query`.
fn search_loop<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

/// Iterator-chain version of the same function -- same result, same
/// asymptotic cost; the compiler inlines the closure and unrolls the loop
/// behind `.filter()`.
fn search_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

// --- .windows() + .fold(): "loop with running state" as an iterator chain ----------

/// Loop version: length of the longest run of strictly increasing elements.
fn longest_run_loop(data: &[i32]) -> usize {
    if data.is_empty() {
        return 0;
    }
    let mut longest = 1;
    let mut current = 1;
    for i in 1..data.len() {
        if data[i] > data[i - 1] {
            current += 1;
        } else {
            current = 1;
        }
        longest = longest.max(current);
    }
    longest
}

/// Iterator-chain version: `.windows(2)` + `.fold()` carries `(longest,
/// current)` as the accumulator, replacing the two `mut` locals above.
fn longest_run_iter(data: &[i32]) -> usize {
    if data.is_empty() {
        return 0;
    }
    data.windows(2)
        .fold((1, 1), |(longest, current), w| {
            let current = if w[1] > w[0] { current + 1 } else { 1 };
            (longest.max(current), current)
        })
        .0
}

// --- f64 min/max via .fold(): f64 doesn't implement Ord -----------------------------

fn min_max(data: &[f64]) -> (f64, f64) {
    let min = data.iter().copied().fold(f64::INFINITY, f64::min);
    let max = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (min, max)
}

// --- .scan(): stateful map, e.g. cumulative sum -------------------------------------

fn cumulative_sum(data: &[i32]) -> Vec<i32> {
    data.iter()
        .scan(0, |total, &x| {
            *total += x;
            Some(*total)
        })
        .collect()
}

// --- .zip() + .flat_map() + .chain(): combining sequences ----------------------------

fn interleave_with_remainder(a: &[i32], b: &[i32]) -> Vec<i32> {
    let common = a.len().min(b.len());
    let zipped = a[..common]
        .iter()
        .zip(b[..common].iter())
        .flat_map(|(&x, &y)| [x, y]);

    let leftover = if a.len() > b.len() {
        a[common..].iter().copied()
    } else {
        b[common..].iter().copied()
    };

    zipped.chain(leftover).collect()
}

fn main() {
    // --- loop vs. iterator chain produce identical results ---
    let contents = "the quick\nbrown fox\njumps over\nthe lazy dog";
    println!("search_loop(\"the\") = {:?}", search_loop("the", contents));
    println!("search_iter(\"the\") = {:?}", search_iter("the", contents));
    assert_eq!(search_loop("the", contents), search_iter("the", contents));

    let data = [1, 2, 3, 2, 3, 4, 5, 1];
    println!(
        "\nlongest_run_loop({data:?}) = {}",
        longest_run_loop(&data)
    );
    println!("longest_run_iter({data:?}) = {}", longest_run_iter(&data));
    assert_eq!(longest_run_loop(&data), longest_run_iter(&data));

    // --- f64 min/max via fold (Ord doesn't exist for f64) ---
    let floats = [3.5, -1.2, 7.8, 0.0, 4.4];
    let (min, max) = min_max(&floats);
    println!("\nmin_max({floats:?}) = ({min}, {max})");

    // --- .scan() for cumulative sum ---
    let nums = [1, 2, 3, 4];
    println!("\ncumulative_sum({nums:?}) = {:?}", cumulative_sum(&nums));

    // --- .zip() + .flat_map() + .chain() for interleaving uneven slices ---
    let a = [1, 3, 5];
    let b = [2, 4];
    println!(
        "\ninterleave_with_remainder({a:?}, {b:?}) = {:?}",
        interleave_with_remainder(&a, &b)
    );
    let a2 = [1, 3];
    let b2 = [2, 4, 6, 8];
    println!(
        "interleave_with_remainder({a2:?}, {b2:?}) = {:?}",
        interleave_with_remainder(&a2, &b2)
    );
}
