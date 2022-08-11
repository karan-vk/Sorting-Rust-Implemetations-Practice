use crate::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            let smart = false;
            if !smart {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                let i = slice[..unsorted]
                    .binary_search(&slice[unsorted])
                    .unwrap_or_else(|x| x);
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn insertionsort_odd_works() {
    let mut v = vec![3, 2, 1];
    InsertionSort::sort(&mut v);
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
fn insertionsort_even_works() {
    let mut v = vec![3, 2, 1, 5];
    InsertionSort::sort(&mut v);
    assert_eq!(v, vec![1, 2, 3, 5]);
}
