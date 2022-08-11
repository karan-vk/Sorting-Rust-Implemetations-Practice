use super::Sorter;

pub struct BubbleSort;

impl<T> Sorter<T> for BubbleSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn bubblesort_works() {
    let mut v = vec![3, 2, 1];
    BubbleSort.sort(&mut v);
    assert_eq!(v, vec![1, 2, 3]);

    let mut v = vec![3, 2, 1, 5];
    BubbleSort.sort(&mut v);
    assert_eq!(v, vec![1, 2, 3, 5]);
}
