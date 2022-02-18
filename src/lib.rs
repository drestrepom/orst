pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord;
}

mod bubblesort;
mod insertionsort;
mod quicksort;
mod selectionsort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;

#[cfg(test)]
mod tests {
    use super::bubblesort::*;
    use super::insertionsort::*;
    use super::quicksort::*;
    use super::selectionsort::*;
    use super::*;

    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(&self, slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }
    #[test]
    fn std_works() {
        let mut things = vec![4, 3, 2, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }
    #[test]
    fn bubblesort_works() {
        let mut things = vec![4, 3, 5, 2, 1];
        BubbleSort.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }
    #[test]
    fn insertionsort_works_smart() {
        let mut things = vec![4, 3, 5, 2, 1];
        InsertionSort { smart: true }.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }
    #[test]
    fn insertionsort_works_no_smart() {
        let mut things = vec![4, 3, 5, 2, 1];
        InsertionSort { smart: false }.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }
    #[test]
    fn selectionsort_works() {
        let mut things = vec![4, 3, 5, 2, 1];
        SelectionSort.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }
    #[test]
    fn quicksort_works() {
        let mut things = vec![4, 3, 5, 2, 1];
        QuickSort.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }
}
