pub mod heap_sort;

#[cfg(test)]
mod tests {
    use crate::heap_sort;
    const LIST: [i32; 12] = [60, 1, 100, 1, 4, 33, 2, 5, 11, 10, 3, 44];

    #[test]
    fn it_works() {
        let result = 2 * 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn heap_test() {
        let sorted = heap_sort::heap_sort(&LIST);
        assert_eq!(sorted[..], [1, 1, 2, 3, 4, 5, 10, 11, 33, 44, 60, 100]);
    }
}
