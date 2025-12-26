// BUCKET SORT - O(n + k)
// Distributes elements into buckets, sorts buckets individually
use super::insertion_sort::insertion_sort;

pub fn bucket_sort(arr: &mut Vec<i32>) {
    if arr.is_empty() {
        return;
    }

    let max_val = *arr.iter().max().unwrap();
    let min_val = *arr.iter().min().unwrap();
    let bucket_count = 10;
    let range = (max_val - min_val + 1) as f64;

    // Create buckets
    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); bucket_count];

    // Distribute elements into buckets
    for &num in arr.iter() {
        let bucket_idx = (((num - min_val) as f64 / range * bucket_count as f64) as usize)
            .min(bucket_count - 1);
        buckets[bucket_idx].push(num);
    }

    // Sort individual buckets and concatenate
    let mut index = 0;
    for bucket in buckets.iter_mut() {
        insertion_sort(bucket);
        for &num in bucket.iter() {
            arr[index] = num;
            index += 1;
        }
    }
}
