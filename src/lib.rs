trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord;
}

fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice)
}

mod bubblesort;
mod insertionsort;

#[cfg(test)]
mod tests {
    use super::*;

    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }

    #[test]
    fn std_works() {
        let mut v = vec![3, 2, 1];
        sort::<_, StdSorter>(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }

    

    
}
