/// Sort array using merge sort algorithm.
pub fn sort<T: PartialOrd + Clone>(a: &mut [T]) {
    if a.len() <= 1 {
        return;
    }

    let q = a.len() / 2;

    let mut merged = a.to_vec();
    let (left, right) = a.split_at_mut(q);

    sort(left);
    sort(right);

    merge(left, right, &mut merged);

    a.clone_from_slice(&merged);
}

/// Merge two adjacent, sorted subarrays.
fn merge<T: PartialOrd + Clone>(left: &[T], right: &[T], merged: &mut [T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged[k] = left[i].clone();
            i += 1;
        } else {
            merged[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    if i < left.len() {
        merged[k..].clone_from_slice(&left[i..])
    }
    if j < right.len() {
        merged[k..].clone_from_slice(&right[j..])
    }
}

/// Sort array using merge sort algorithm.
/// *A* is an array and *p* & *r* are indices into the array such that *p < r*.
pub fn sort_book<T: PartialOrd + Clone>(a: &mut [T], p: usize, r: usize) {
    if p + 1 >= r {
        return;
    }

    let q = (p + r) / 2;

    sort_book(a, p, q);
    sort_book(a, q + 1, r);

    merge_book(a, p, q, r);
}

/// `left`and right*A* is an array and *p*, *q*, *r* are indices into the array such that *p <= q < r*.
fn merge_book<T: PartialOrd + Clone>(a: &mut [T], p: usize, q: usize, r: usize) {
    let nl = q - p + 1;
    let nr = r - q;

    let left = &a[p..=q].to_owned();
    let right = &a[q + 1..=r].to_owned();

    let mut i = 0;
    let mut j = 0;
    let mut k = p;

    while i < nl && j < nr {
        if left[i] <= right[j] {
            a[k] = left[i].clone();
            i += 1;
        } else {
            a[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < nl {
        a[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < nr {
        a[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{sort, sort_book};

    #[test]
    fn test_merge_sort() {
        let mut arr = [2, 4, 6, 7, 1, 2, 3, 5];

        sort(&mut arr);

        assert_eq!(arr, [1, 2, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_merge_sort_book() {
        let mut arr = [2, 4, 6, 7, 1, 2, 3, 5];

        sort_book(&mut arr, 0, 7);

        assert_eq!(arr, [1, 2, 2, 3, 4, 5, 6, 7]);
    }
}
