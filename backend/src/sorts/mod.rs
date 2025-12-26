// ============================================================================
// SORTING ALGORITHMS MODULE
// ============================================================================

pub mod bubble_sort;
pub mod selection_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod heap_sort;
pub mod tim_sort;
pub mod radix_sort;
pub mod bucket_sort;
pub mod shell_sort;

// Re-export all sorting functions for convenient access
pub use bubble_sort::bubble_sort;
pub use selection_sort::selection_sort;
pub use insertion_sort::insertion_sort;
pub use merge_sort::merge_sort;
pub use quick_sort::quick_sort;
pub use heap_sort::heap_sort;
pub use tim_sort::tim_sort;
pub use radix_sort::radix_sort;
pub use bucket_sort::bucket_sort;
pub use shell_sort::shell_sort;

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_all_sorts() {
        let test_data = vec![64, 34, 25, 12, 22, 11, 90, 88, 45, 50];

        let sorts: Vec<(&str, fn(&mut Vec<i32>))> = vec![
            ("Bubble Sort", bubble_sort),
            ("Selection Sort", selection_sort),
            ("Insertion Sort", insertion_sort),
            ("Merge Sort", merge_sort),
            ("Quick Sort", quick_sort),
            ("Heap Sort", heap_sort),
            ("Tim Sort", tim_sort),
            ("Radix Sort", radix_sort),
            ("Bucket Sort", bucket_sort),
            ("Shell Sort", shell_sort),
        ];

        for (name, sort_fn) in sorts {
            let mut data = test_data.clone();
            sort_fn(&mut data);
            assert!(is_sorted(&data), "{} failed to sort correctly", name);
        }
    }
}
