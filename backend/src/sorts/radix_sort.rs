// RADIX SORT - O(nk)
// Non-comparison based sort that sorts numbers digit by digit
pub fn radix_sort(arr: &mut Vec<i32>) {
    if arr.is_empty() {
        return;
    }

    let max_val = *arr.iter().max().unwrap();
    let mut exp = 1;

    while max_val / exp > 0 {
        counting_sort_by_digit(arr, exp);
        exp *= 10;
    }
}

fn counting_sort_by_digit(arr: &mut Vec<i32>, exp: i32) {
    let n = arr.len();
    let mut output = vec![0; n];
    let mut count = vec![0; 10];

    // Store count of occurrences
    for &num in arr.iter() {
        let digit = ((num / exp) % 10) as usize;
        count[digit] += 1;
    }

    // Change count[i] to actual position
    for i in 1..10 {
        count[i] += count[i - 1];
    }

    // Build output array
    for i in (0..n).rev() {
        let digit = ((arr[i] / exp) % 10) as usize;
        output[count[digit] - 1] = arr[i];
        count[digit] -= 1;
    }

    // Copy output array to arr
    arr.copy_from_slice(&output);
}
