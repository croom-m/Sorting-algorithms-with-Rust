mod buble_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod selection_sort;

fn main() {
    let arr: &mut [i32] = &mut [5, 4, 3, 2, 1];
    // buble_sort::buble_sort(arr);
    // insertion_sort::insertion_sort(arr);
    // selection_sort::selection_sort(arr);
    // merge_sort::merge_sort(arr);
    quick_sort::quick_sort(arr);
    println!("{:?}", arr);
}
