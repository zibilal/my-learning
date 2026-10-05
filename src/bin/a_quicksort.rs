fn partition<T:Ord>(slice: &mut[T]) -> usize {
    let len = slice.len();
    let pivot_index = len -1; // use the last element as pivot
    let mut i = 0;

    for j in 0..pivot_index {
        if slice[j] <= slice[pivot_index] {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, pivot_index);
    i
}

fn quicksort<T:Ord>(slice: &mut[T])  {
    if slice.len() <= 1 {
        return; // nothing to sort
    }

    // Partition the slice into two parts, front and back
    let pivot_index = partition(slice);

    // Recursively sort the front half of `slice`
    quicksort(&mut slice[..pivot_index]);

    // And the back half
    quicksort(&mut slice[pivot_index + 1..]);
}
fn main() {
    let mut v = vec![5, 3, 8, 4, 2, 7, 1, 10, 6, 9];
    quicksort(&mut v);
    println!("{:?}", v);
}