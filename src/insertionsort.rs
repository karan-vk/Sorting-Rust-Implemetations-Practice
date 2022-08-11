use crate::Sorter;

pub struct InsertionSort {
    smart: bool,
}

impl<T> Sorter<T> for InsertionSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            if !self.smart {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn insertionsort_odd_works() {
    let mut v = vec![3, 2, 1];
    InsertionSort { smart: false }.sort(&mut v);
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
fn insertionsort_even_works() {
    let mut v = vec![3, 2, 1, 5];
    InsertionSort { smart: true }.sort(&mut v);
    assert_eq!(v, vec![1, 2, 3, 5]);
}
