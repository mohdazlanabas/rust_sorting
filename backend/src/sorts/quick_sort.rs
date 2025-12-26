// QUICK SORT - O(n log n) average
// Divide-and-conquer using pivot element to partition array
pub fn quick_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    quick_sort_helper(arr, 0, len - 1);
}

fn quick_sort_helper(arr: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let pi = partition(arr, low, high);

        if pi > 0 {
            quick_sort_helper(arr, low, pi - 1);
        }
        quick_sort_helper(arr, pi + 1, high);
    }
}

fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}
