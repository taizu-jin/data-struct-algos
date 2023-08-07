pub fn sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;

        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort;

    #[test]
    fn test_insertion_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];

        sort(&mut arr);

        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
