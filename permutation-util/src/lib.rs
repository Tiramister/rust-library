pub fn next_permutation<T>(a: &mut [T]) -> bool
where
    T: Ord,
{
    let n = a.len();

    let left = (0..(n - 1)).rev().find(|&i| a[i] < a[i + 1]).unwrap_or(n);
    if left == n {
        a.reverse();
        return false;
    }

    let right = ((left + 1)..n).rev().find(|&i| a[left] < a[i]).unwrap();

    a.swap(left, right);
    a[(left + 1)..].reverse();
    true
}
