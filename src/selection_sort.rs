pub fn selection_sort(a: &mut [i32]) {
    for i in 0..a.len() {
        let mut min = i;
        for j in i + 1..a.len() {
            if a[min] > a[j] {
                min = j;
            }
        }
        a.swap(i, min);
    }
}
