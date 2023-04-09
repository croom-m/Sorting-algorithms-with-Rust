pub fn buble_sort(arr: &mut [i32]) {
    for _i in 1..arr.len() {
        for j in 1..arr.len() {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j)
            }
        }
    }
}
