pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;
}

// fn sort<T, S>(slice: &mut [T])
// where
//     T: Ord,
//     S: Sorter<T>,
// {
//     S::sort(slice)
// }

mod bubblesort;
mod insertionsort;

#[cfg(test)]
mod tests {
    use super::*;

    struct StdSorter;
    impl<T> Sorter<T> for StdSorter {
        fn sort(&self, slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }

    #[test]
    fn std_works() {
        let mut v = vec![3, 2, 1];
        StdSorter.sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }
}
