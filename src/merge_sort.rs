pub fn merge_sort(arr: &mut [i32]) {
    merge_sort_helper(arr, 0, arr.len());
}

fn merge(arr: &mut [i32], l: usize, m: usize, r: usize) {
    let mut a = 0;
    let mut b = 0;
    let mut i = 0;
    let mut arr2 = vec![0; r - l];
    while (l + a < m) && (m + b < r) {
        if arr[l + a] < arr[m + b] {
            arr2[i] = arr[l + a];
            a = a + 1;
            i = i + 1;
        } else {
            arr2[i] = arr[m + b];
            b = b + 1;
            i = i + 1;
        }
    }

    if l + a == m {
        for _j in m + b..r {
            arr2[i] = arr[_j];
            i = i + 1;
        }
    }

    if m + b == r {
        for _j in l + a..m {
            arr2[i] = arr[_j];
            i = i + 1;
        }
    }

    for _i in 0..arr2.len() {
        arr[l + _i] = arr2[_i];
    }
}

fn merge_sort_helper(arr: &mut [i32], l: usize, r: usize) {
    let left = l;
    let right = r;
    if right - left > 1 {
        let middle = left + (right - left) / 2;
        merge_sort_helper(arr, left, middle);
        merge_sort_helper(arr, middle, right);
        merge(arr, left, middle, right);
    }
}
