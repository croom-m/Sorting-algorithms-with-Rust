pub fn quick_sort(arr: &mut [i32]) {
    quick_sort_helper(arr, 0, arr.len());
}

fn quick_sort_helper(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pi = partition(arr, low, high);
        quick_sort_helper(arr, low, pi);
        quick_sort_helper(arr, pi + 1, high);
    }
}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
  let mut min = low;
  let pivot = high - 1;
  for i in low..pivot {
    if arr[i] < arr[pivot] {
      arr.swap(i, min);
      min += 1;
    }
  }
  arr.swap(min, pivot);
  return min;
}