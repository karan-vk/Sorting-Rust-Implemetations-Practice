use crate::Sorter;

pub struct SelectionSort;

impl<T> Sorter<T> for SelectionSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for i in 0..slice.len() {
            let mut min = i;
            for j in i..slice.len() {
                if slice[j] < slice[min] {
                    min = j;
                }
            }
            slice.swap(i, min);
        }
    }
}

#[test]
fn insertionsort_odd_works() {
    let mut v = vec![3, 2, 1];
    SelectionSort.sort(&mut v);
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
fn insertionsort_even_works() {
    let mut v = vec![3, 2, 1, 5];
    SelectionSort.sort(&mut v);
    assert_eq!(v, vec![1, 2, 3, 5]);
}
