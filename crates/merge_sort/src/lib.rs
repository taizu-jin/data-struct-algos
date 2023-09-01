/// Sort array using merge sort algorithm.
/// *A* is an array and *p* & *r* are indices into the array such that *p < r*.
pub fn sort<T: PartialOrd + Clone>(a: &mut [T], p: usize, r: usize) {
    if p + 1 >= r {
        return;
    }

    let q = (p + r) / 2;

    sort(a, p, q);
    sort(a, q + 1, r);

    merge(a, p, q, r);
}

/// Merge two adjacent, sorted subarrays.
/// *A* is an array and *p*, *q*, *r* are indices into the array such that *p <= q < r*.
fn merge<T: PartialOrd + Clone>(a: &mut [T], p: usize, q: usize, r: usize) {
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
    use crate::sort;

    #[test]
    fn test_merge_sort() {
        let mut arr = [2, 4, 6, 7, 1, 2, 3, 5];

        sort(&mut arr, 0, 7);

        assert_eq!(arr, [1, 2, 2, 3, 4, 5, 6, 7]);
    }
}
