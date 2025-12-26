// SHELL SORT - O(n log²n)
// Generalization of insertion sort that allows exchange of far items
pub fn shell_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    let mut gap = n / 2;

    while gap > 0 {
        for i in gap..n {
            let temp = arr[i];
            let mut j = i;

            while j >= gap && arr[j - gap] > temp {
                arr[j] = arr[j - gap];
                j -= gap;
            }
            arr[j] = temp;
        }
        gap /= 2;
    }
}
