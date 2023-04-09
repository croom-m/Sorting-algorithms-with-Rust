pub fn insertion_sort(arr: &mut [i32]) {
    let mut i = 1;
    while i < arr.len() {
        let mut j = i;
        while arr[j] < arr[j - 1] {
            arr.swap(j - 1, j);
            if j == 1 {
                break;
            } else {
                j = j - 1
            }
        }
        i += 1;
    }
}
