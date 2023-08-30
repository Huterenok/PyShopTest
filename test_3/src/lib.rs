mod tests;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use sha256::digest;

fn hash_value(value: u64) -> String {
    digest(value.to_string())
}

/// Checks amount of zeroes in the end of value and returns true if it has right amount
fn check_zeroes(value: &str, amount: u8) -> bool {
    let zeroes = "0".repeat(amount as usize);
    value.ends_with(&zeroes)
}

/// Creates chunks with numbers for threads
fn u64_range_chunks(size: u64) -> impl Iterator<Item = std::ops::Range<u64>> {
    (0..u64::MAX / size).map(move |val| val * size..(val + 1) * size)
}

/// Finds hashes using with rayon on each chunk from u64_range_chunks.
/// Flattens returned vecs with good hashes and println f amount hashes.
pub fn find_hashes(n: u8, f: u8) -> Vec<(u64, String)> {
    u64_range_chunks(100000)
        .map(|range| {
            range
                .into_par_iter()
                .filter_map(|x| {
                    let hash = hash_value(x);
                    if check_zeroes(&hash, n) {
                        Some((x, hash))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(u64, String)>>()
        })
        .flatten()
        .take(f.into())
        .collect()
}
