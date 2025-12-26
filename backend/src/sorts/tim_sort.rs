// TIM SORT - O(n log n)
// Hybrid sorting algorithm derived from merge sort and insertion sort
pub fn tim_sort(arr: &mut Vec<i32>) {
    const RUN: usize = 32;
    let n = arr.len();

    // Sort individual subarrays of size RUN
    for start in (0..n).step_by(RUN) {
        let end = std::cmp::min(start + RUN, n);
        insertion_sort_range(arr, start, end);
    }

    // Merge sorted runs
    let mut size = RUN;
    while size < n {
        for start in (0..n).step_by(size * 2) {
            let mid = start + size;
            let end = std::cmp::min(start + size * 2, n);

            if mid < end {
                let left = arr[start..mid].to_vec();
                let right = arr[mid..end].to_vec();
                merge_range(arr, start, &left, &right);
            }
        }
        size *= 2;
    }
}

fn insertion_sort_range(arr: &mut Vec<i32>, start: usize, end: usize) {
    for i in (start + 1)..end {
        let key = arr[i];
        let mut j = i;
        while j > start && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn merge_range(arr: &mut Vec<i32>, start: usize, left: &[i32], right: &[i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = start;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}
