// HEAP SORT - O(n log n)
// Builds a max heap and repeatedly extracts maximum element
pub fn heap_sort(arr: &mut Vec<i32>) {
    let n = arr.len();

    // Build max heap
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // Extract elements from heap one by one
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify(arr: &mut Vec<i32>, n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}
