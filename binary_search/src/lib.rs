pub trait BinarySearch<T> {
    /// x 以上の最小の要素のインデックスを返す
    /// 全ての要素が x より小さい場合は self.len() を返す
    fn lower_bound(&self, x: &T) -> usize;

    /// x より大きい最小の要素のインデックスを返す
    /// 全ての要素が x 以下の場合は self.len() を返す
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T> BinarySearch<T> for [T]
where
    T: Ord,
{
    fn lower_bound(&self, x: &T) -> usize {
        if self.is_empty() || self[0] >= *x {
            return 0;
        }

        // self[low] < x <= self[high]
        let mut low = 0;
        let mut high = self.len();
        while high - low > 1 {
            let mid = (low + high) / 2;
            if self[mid] < *x {
                low = mid;
            } else {
                high = mid;
            }
        }
        high
    }

    fn upper_bound(&self, x: &T) -> usize {
        if self.is_empty() || self[0] > *x {
            return 0;
        }

        // self[low] <= x < self[high]
        let mut low = 0;
        let mut high = self.len();
        while high - low > 1 {
            let mid = (low + high) / 2;
            if self[mid] <= *x {
                low = mid;
            } else {
                high = mid;
            }
        }
        high
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_bound_empty() {
        assert_eq!(vec![].lower_bound(&1), 0);
    }

    #[test]
    fn test_lower_bound() {
        let v = vec![2, 2, 4, 6, 6, 6, 8];
        let expects = vec![0, 0, 0, 2, 2, 3, 3, 6, 6, 7];
        for (i, expect) in expects.iter().enumerate() {
            assert_eq!(v.lower_bound(&i), *expect);
        }
    }

    #[test]
    fn test_upper_bound_empty() {
        assert_eq!(vec![].lower_bound(&1), 0);
    }

    #[test]
    fn test_upper_bound() {
        let v = vec![2, 2, 4, 6, 6, 6, 8];
        let expects = vec![0, 0, 2, 2, 3, 3, 6, 6, 7, 7];
        for (i, expect) in expects.iter().enumerate() {
            assert_eq!(v.upper_bound(&i), *expect);
        }
    }
}
